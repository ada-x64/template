use bevy::ecs::{
    component::{ComponentHook, HookContext, Immutable, StorageType},
    world::DeferredWorld,
};
#[allow(unused_imports, reason = "used for docs")]
use bevy::{
    app::{FixedMain, FixedMainScheduleOrder, MainScheduleOrder},
    ecs::{schedule::ScheduleLabel, system::ScheduleSystem},
};

pub use crate::prelude::*;

/// On build, this will initialize a new [Schedule]. The newly created schedule
/// has a [SystemSet] associated with it which is scoped to run only if the
/// world is in the given ReadyState. Schedules can run in either [Main] or
/// [FixedMain]. By default, the given systems will run after [Update] or
/// [FixedUpdate], but this can be configured by calling
/// [ScreenScopeBuilder::build_with_order] /
/// [ScreenScopeBuilder::build_with_order_fixed].
pub struct ScreenScopeBuilder<S>
where
    S: Screen,
{
    schedule: Schedule,
    scope: ScreenScope<S>,
    fixed: bool,
}

impl<S> Default for ScreenScopeBuilder<S>
where
    S: Screen,
{
    fn default() -> Self {
        Self::new_inner(ScreenScope::<S>::Update, false)
    }
}
impl<S> ScreenScopeBuilder<S>
where
    S: Screen,
{
    fn new_inner(scope: ScreenScope<S>, fixed: bool) -> Self {
        let mut schedule = Schedule::new(scope);
        schedule.configure_sets(scope.run_if(in_state(CurrentScreen::new::<S>())));
        Self {
            schedule,
            scope,
            fixed,
        }
    }

    pub fn fixed() -> Self {
        Self::new_inner(ScreenScope::<S>::FixedUpdate, true)
    }

    /// Add systems to the schedule scope.
    /// In order to scope observers, use `on_enter`
    pub fn add_systems<M>(mut self, systems: impl IntoScheduleConfigs<ScheduleSystem, M>) -> Self {
        self.schedule.add_systems(systems.in_set(self.scope));
        self
    }

    /// Builds the schedule. It will run after [Update], or [FixedUpdate] if
    /// fixed.
    pub fn build(self, app: &mut App) {
        if self.fixed {
            self.build_inner(app, Order::After(FixedUpdate));
        } else {
            self.build_inner(app, Order::After(Update));
        };
    }

    fn build_inner<L: ScheduleLabel>(self, app: &mut App, order: Order<L>) {
        app.add_schedule(self.schedule);
        app.add_systems(
            OnEnter(CurrentScreen::new::<S>()),
            |mut commands: Commands| {
                commands.spawn(ScreenWrapper(S::default()));
            },
        );
        app.add_systems(
            OnExit(CurrentScreen::new::<S>()),
            |mut commands: Commands, e: Single<Entity, With<ScreenWrapper<S>>>| {
                commands.entity(*e).despawn();
            },
        );
        if self.fixed {
            let mut ms_order = app.world_mut().resource_mut::<FixedMainScheduleOrder>();
            match order {
                Order::Before(l) => ms_order.insert_before(l, self.scope),
                Order::After(l) => ms_order.insert_after(l, self.scope),
            }
        } else {
            let mut ms_order = app.world_mut().resource_mut::<MainScheduleOrder>();
            match order {
                Order::Before(l) => ms_order.insert_before(l, self.scope),
                Order::After(l) => ms_order.insert_after(l, self.scope),
            }
        }
    }
}

/// Specifies the order of execution for a schedule.
/// See [Main] and [FixedMain] for default schedule orders.
pub enum Order<L: ScheduleLabel> {
    Before(L),
    After(L),
}

/// Implementation trait for Screen components.
/// ## Lifecycle
/// Screens have two lifecycle functions.
///
/// The first is [Screen::init]
/// which is called when the screen component's `on_add` hook is fired.
/// This is meant to register systems and scoped observers.
///
/// The second lifecycle function is [Screen::unload]. This function
/// is called before the screen unloads and is designed to run
/// any cleanup logic before transitioning.
pub trait Screen:
    Sized + Default + std::fmt::Debug + Clone + Copy + Eq + std::hash::Hash + Send + Sync + 'static
{
    /// The associated screen name.
    const NAME: Screens;

    /// Called during the on_add hook.
    /// Use this to scope systems and observers.
    fn init<'w>(_: &mut DeferredWorld<'w>, _: &HookContext) {}
    /// Called when the screen is about to unload.
    /// Use this to perform any necessary cleanup before the screen transitions.
    fn unload(_: &mut World) {}

    /// INTERNAL
    fn on_unload(_trigger: Trigger<SwitchScreen>, mut commands: Commands) {
        info!("on_unload");
        commands.queue(Self::unload);
        commands.queue(|world: &mut World| world.trigger(FinishUnload));
    }

    /// Creates a new [ScreenScopeBuilder] which will run in [Main] after
    /// [Update]. To run on a fixed schedule, call [Self::builder_fixed] instead.
    fn builder() -> ScreenScopeBuilder<Self> {
        ScreenScopeBuilder::<Self>::default()
    }
    /// Creates a new [ScreenScopeBuilder] which will run in [FixedMain] after
    /// [FixedUpdate]. To run on a non-fixed schedule, use [Self::builder] instead.
    fn builder_fixed() -> ScreenScopeBuilder<Self> {
        ScreenScopeBuilder::<Self>::fixed()
    }
}

pub struct ScreenWrapper<T: Screen>(T);

impl<T> Component for ScreenWrapper<T>
where
    T: Screen,
{
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Immutable;

    fn on_add() -> Option<ComponentHook> {
        info!("ScreenWrapper::on_add");
        Some(|mut world: DeferredWorld, ctx: HookContext| {
            T::init(&mut world, &ctx);
            world.commands().entity(ctx.entity).observe(T::on_unload);
        })
    }
}

use std::marker::PhantomData;

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
#[derive(Default)]
pub struct ScreenScope<S: Screen> {
    c: PhantomData<S>,
}
impl<S: Screen> ScreenScope<S> {
    pub fn builder<Scope, ReadyState>(
        self,
        scope: Scope,
        ready_state: ReadyState,
    ) -> ScreenScopeBuilder<S, Scope, ReadyState>
    where
        Scope: SystemSet + Clone + ScheduleLabel,
        ReadyState: States,
    {
        ScreenScopeBuilder::new(scope, ready_state)
    }
}

/// A utility for scoping systems to a particular state.
/// See [ScreenScope] for more details.
pub struct ScreenScopeBuilder<S, Scope, ReadyState>
where
    S: Screen,
    Scope: SystemSet + ScheduleLabel + Clone,
    ReadyState: States,
{
    schedule: Schedule,
    scope: Scope,
    component: PhantomData<S>,
    ready_state: ReadyState,
}
impl<S, Scope, ReadyState> ScreenScopeBuilder<S, Scope, ReadyState>
where
    S: Screen,
    Scope: SystemSet + ScheduleLabel + Clone,
    ReadyState: States,
{
    pub fn new(scope: Scope, ready_state: ReadyState) -> Self {
        let mut schedule = Schedule::new(scope.clone());
        schedule.configure_sets(scope.clone().run_if(in_state(ready_state.clone())));
        Self {
            schedule,
            component: PhantomData,
            ready_state,
            scope,
        }
    }

    /// Add systems to the schedule scope.
    /// In order to scope observers, use `on_enter`
    pub fn add_systems<M>(mut self, systems: impl IntoScheduleConfigs<ScheduleSystem, M>) -> Self {
        self.schedule
            .add_systems(systems.in_set(self.scope.clone()));
        self
    }

    /// Builds the schedule. It will run after [Update].
    pub fn build(self, app: &mut App) {
        self.build_inner(app, Order::After(Update), false);
    }
    /// Builds the schedule. It will run after [FixedUpdate].
    pub fn build_fixed(self, app: &mut App) {
        self.build_inner(app, Order::After(FixedUpdate), true);
    }
    /// Builds the schedule. It will in [Main] according to the specified [Order].
    pub fn build_with_order<L: ScheduleLabel>(self, app: &mut App, order: Order<L>) {
        self.build_inner(app, order, false);
    }
    /// Builds the schedule. It will in [FixedMain] according to the specified [Order].
    pub fn build_with_order_fixed<L: ScheduleLabel>(self, app: &mut App, order: Order<L>) {
        self.build_inner(app, order, true);
    }

    fn build_inner<L: ScheduleLabel>(self, app: &mut App, order: Order<L>, fixed: bool) {
        app.add_schedule(self.schedule);
        app.add_systems(
            OnEnter(self.ready_state.clone()),
            |mut commands: Commands| {
                commands.spawn(ScreenWrapper(S::default()));
            },
        );
        app.add_systems(
            OnExit(self.ready_state),
            |mut commands: Commands, e: Single<Entity, With<ScreenWrapper<S>>>| {
                commands.entity(*e).despawn();
            },
        );
        if fixed {
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
pub trait Screen: Sized + Default + Send + Sync + 'static {
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

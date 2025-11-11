use std::marker::PhantomData;

#[allow(unused_imports, reason = "used in docs")]
use bevy::app::FixedMain;
use bevy::{
    app::{FixedMainScheduleOrder, MainScheduleOrder},
    ecs::{schedule::ScheduleLabel, system::ScheduleSystem},
};

pub use crate::prelude::*;

/// On build, this will initialize a new [Schedule]. The newly created schedule
/// has a [SystemSet] associated with it which is scoped to run only if the
/// world is in the given ReadyState. Schedules can run in either [Main] or
/// [FixedMain]. The given systems will run after [Update] or
/// [FixedUpdate].
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
        let schedule = Schedule::new(scope);
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
        debug!("Building screen '{:?}'", S::name());

        // insert data
        app.add_schedule(self.schedule);
        app.init_resource::<S::SETTINGS>();

        // configure screen switch event
        app.add_systems(
            OnEnter(CurrentScreen(S::name())),
            |mut commands: Commands| {
                debug!("OnEnter({:?})", S::name());
                commands.spawn(ScreenWrapper(S::default()));
            },
        );
        app.add_systems(
            OnExit(CurrentScreen(S::name())),
            |mut commands: Commands, e: Single<Entity, With<ScreenWrapper<S>>>| {
                debug!("OnExit({:?})", S::name());
                commands.entity(*e).despawn();
            },
        );
        app.add_observer(on_switch_screen::<S>);

        // scope systems
        app.configure_sets(
            self.scope,
            self.scope.run_if(in_state(CurrentScreen(S::name()))),
        );

        // add to main schedule
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

        // configure unload
        app.add_schedule(Schedule::new(UnloadSchedule::<S>::default()));
        app.add_systems(
            UnloadSchedule::<S>::default(),
            (S::unload(), on_finish_unload).run_if(in_state(CurrentScreen(S::name()))),
        );
    }
}

/// Specifies the order of execution for a schedule.
/// See [Main] and [FixedMain] for default schedule orders.
pub enum Order<L: ScheduleLabel> {
    Before(L),
    After(L),
}

// Manuallly triggered schedule which is called when the screen is unloaded.
#[derive(ScheduleLabel, Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
struct UnloadSchedule<T: Screen>(PhantomData<T>);

// TODO: Would be nice to not have duplicates of this function.
// Unfortunately, it seems impossible to convert from Screens to impl Screen
// (Screen is not dyn compatible)
fn on_switch_screen<T: Screen>(
    trigger: On<SwitchToScreen>,
    mut commands: Commands,
    current_screen: Res<State<CurrentScreen>>,
    mut next_screen: ResMut<NextScreen>,
    mut next_state: ResMut<NextState<CurrentScreenStatus>>,
) {
    debug!("on_switch_screen ({:?})", T::name());
    if ***current_screen == T::name() {
        next_state.set(CurrentScreenStatus(ScreenStatus::Unloading));
        *next_screen = NextScreen(Some(trigger.0));
        commands.run_schedule(UnloadSchedule::<T>::default());
    }
}

/// This function clears out all the non-screen-scoped entities.
fn on_finish_unload(
    mut next_screen: ResMut<NextScreen>,
    mut current_screen: ResMut<NextState<CurrentScreen>>,
    mut current_status: ResMut<NextState<CurrentScreenStatus>>,
    // Any entity which is (explicitly marked as ScreenScoped, or is _not_ marked
    // as persistent) _and_ is not a top-level observer
    screen_scoped: Query<
        Entity,
        (
            Or<(
                With<ScreenScoped>,  // is explicitly screen-scoped
                Without<Persistent>, // is explicitly persistent
            )>,
            Not<(Or<(With<Observer>, With<Window>)>, Without<ChildOf>)>, // top-level items
        ),
    >,
    mut commands: Commands,
) {
    debug!("on_finish_unload");
    current_screen.set(next_screen.0.take().unwrap().into());
    current_status.set(ScreenStatus::Loading.into());
    screen_scoped.iter().for_each(|e| {
        commands.entity(e).despawn();
    });
}

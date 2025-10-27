use bevy::{
    app::{FixedMainScheduleOrder, MainScheduleOrder},
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
        app.add_schedule(self.schedule);
        app.init_resource::<S::SETTINGS>();
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

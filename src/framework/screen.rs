use std::marker::PhantomData;

#[allow(unused_imports, reason = "used for docs")]
use bevy::{
    app::{FixedMain, FixedMainScheduleOrder, MainScheduleOrder},
    ecs::{schedule::ScheduleLabel, system::ScheduleSystem},
};

pub use crate::prelude::*;

/// A utility for scoping systems to a particular state.
/// See [SceenScopeBuilder] for more details.
#[derive(Default)]
pub struct ScreenScope<C: Component + Default> {
    c: PhantomData<C>,
}
impl<C: Component + Default> ScreenScope<C> {
    pub fn builder<Scope, ReadyState>(
        self,
        scope: Scope,
        ready_state: ReadyState,
    ) -> ScreenScopeBuilder<C, Scope, ReadyState>
    where
        Scope: SystemSet + Clone + ScheduleLabel,
        ReadyState: States,
    {
        ScreenScopeBuilder::new(scope, ready_state)
    }
}

/// On build, this will initialize a new [Schedule].
/// The newly created schedule has a [SystemSet] associated with it which is
/// scoped to run only if the world is in the given ReadyState.
/// Schedules can run in either [Main] or [FixedMain].
/// By default, the given systems will run after [Update] or [FixedUpdate],
/// but this can be configured by calling [Self::build_with_order] / [Self::build_with_order_fixed].
pub struct ScreenScopeBuilder<C, Scope, ReadyState>
where
    C: Component + Default,
    Scope: SystemSet + ScheduleLabel + Clone,
    ReadyState: States,
{
    schedule: Schedule,
    scope: Scope,
    component: PhantomData<C>,
    ready_state: ReadyState,
}
impl<Screen, Scope, ReadyState> ScreenScopeBuilder<Screen, Scope, ReadyState>
where
    Screen: Component + Default,
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
    /// In order to scope observers, add them to the screen component's
    /// `on_add` hook.
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
                commands.spawn(Screen::default());
            },
        );
        app.add_systems(
            OnExit(self.ready_state),
            |mut commands: Commands, e: Single<Entity, With<Screen>>| {
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

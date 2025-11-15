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
pub struct ScreenScopeBuilder<'a, S>
where
    S: Screen,
{
    schedule: Schedule,
    fixed_schedule: Schedule,
    scope: ScreenScope<S>,
    fixed_scope: ScreenScope<S>,
    app: &'a mut App,
    order: Order,
    fixed_order: Order,
}

impl<'a, S> ScreenScopeBuilder<'a, S>
where
    S: Screen,
{
    pub fn new(app: &'a mut App) -> Self {
        let scope = ScreenScope::<S>::Main;
        let fixed_scope = ScreenScope::<S>::Fixed;
        Self {
            schedule: Schedule::new(scope),
            fixed_schedule: Schedule::new(fixed_scope),
            scope,
            fixed_scope,
            order: Order::default(),
            fixed_order: Order::default(),
            app,
        }
    }

    /// Sets when this screen's systems run relative to the [Update] in the [Main] schedule
    pub fn with_order(mut self, order: Order) -> Self {
        self.order = order;
        self
    }

    /// Sets when this screen's fixed systems run relative to the [FixedMain]
    /// schedule
    pub fn with_fixed_order(mut self, order: Order) -> Self {
        self.fixed_order = order;
        self
    }

    /// Add systems to the schedule scope. Will run before or after [Update]
    /// according to the builder's [Order]
    pub fn add_systems<M>(mut self, systems: impl IntoScheduleConfigs<ScheduleSystem, M>) -> Self {
        self.schedule.add_systems(systems.in_set(self.scope));
        self
    }

    /// Add systems to the fixed schedule scope. Will run before or after
    /// [FixedUpdate] according to the builder's [Order]
    pub fn add_systems_fixed<M>(
        mut self,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> Self {
        self.fixed_schedule.add_systems(systems.in_set(self.scope));
        self
    }

    /// Adds a system which will run when the screen finishes loading its
    /// systems. This is a shorthand for
    /// `app.add_systems(OnEnter(ScreenLoadingState::<S>::Ready), systems)`.
    pub fn on_ready<M>(self, systems: impl IntoScheduleConfigs<ScheduleSystem, M>) -> Self {
        self.app
            .add_systems(OnEnter(ScreenLoadingState::<S>::Ready), systems);
        self
    }

    /// Builds the schedule and adds it to the app.
    pub fn build(self) {
        let app = self.app;
        debug!("Building screen '{:?}'", S::name());

        // insert data
        app.add_schedule(self.schedule);
        app.init_resource::<S::SETTINGS>();

        // configure screen switch event
        app.add_systems(
            OnEnter(CurrentScreen(S::name())),
            |mut commands: Commands| {
                debug!("OnEnter({:?})", S::name());
                commands.spawn(S::default());
            },
        );
        app.add_systems(
            OnExit(CurrentScreen(S::name())),
            |mut commands: Commands, e: Single<Entity, With<S>>| {
                debug!("OnExit({:?})", S::name());
                commands.entity(*e).despawn();
            },
        );
        app.add_observer(on_switch_screen::<S>);

        // scope systems
        let (config, fixed_config) = match S::strategy() {
            LoadingStrategy::Blocking => {
                let condition = in_state(CurrentScreen(S::name()))
                    .and(in_state(ScreenLoadingState::<S>::Ready));
                (
                    self.scope.run_if(condition.clone()),
                    self.fixed_scope.run_if(condition),
                )
            }
            LoadingStrategy::Nonblocking => {
                let condition = in_state(CurrentScreen(S::name()));
                (
                    self.scope.run_if(condition.clone()),
                    self.fixed_scope.run_if(condition),
                )
            }
        };
        app.configure_sets(self.scope, config);
        app.configure_sets(self.fixed_scope, fixed_config);

        // add to fixed main
        let mut ms_order = app.world_mut().resource_mut::<FixedMainScheduleOrder>();
        match self.fixed_order {
            Order::Before => ms_order.insert_before(FixedUpdate, self.fixed_scope),
            Order::After => ms_order.insert_after(FixedUpdate, self.fixed_scope),
        }

        // add to main schedule
        let mut ms_order = app.world_mut().resource_mut::<MainScheduleOrder>();
        match self.order {
            Order::Before => ms_order.insert_before(Update, self.scope),
            Order::After => ms_order.insert_after(Update, self.scope),
        }

        // loading state
        app.init_state::<ScreenLoadingState<S>>();
        app.add_loading_state(
            LoadingState::new(ScreenLoadingState::<S>::Loading)
                .continue_to_state(ScreenLoadingState::<S>::Ready)
                .load_collection::<S::ASSETS>(),
        );

        // configure unload
        app.add_schedule(Schedule::new(UnloadSchedule::<S>::default()));
        app.add_systems(
            UnloadSchedule::<S>::default(),
            (S::unload(), on_finish_unload).run_if(in_state(CurrentScreen(S::name()))),
        );
    }
}

/// Specifies the order of execution for a schedule.
#[derive(Default, Debug)]
pub enum Order {
    #[default]
    Before,
    After,
}

/// Manuallly triggered schedule which is called when the screen is unloaded.
#[derive(ScheduleLabel, Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
struct UnloadSchedule<T: Screen>(PhantomData<T>);

// TODO: Would be nice to not have duplicates of this function.
// Unfortunately, it seems impossible to convert from Screens to impl Screen
// (Screen is not dyn compatible)
fn on_switch_screen<S: Screen>(
    trigger: On<SwitchToScreen>,
    mut commands: Commands,
    current_screen: Res<State<CurrentScreen>>,
    mut next_loading_state: ResMut<NextState<ScreenLoadingState<S>>>,
    mut next_screen: ResMut<NextScreen>,
    mut next_state: ResMut<NextState<CurrentScreenStatus>>,
) {
    debug!("on_switch_screen ({:?})", S::name());
    if ***current_screen == S::name() {
        next_state.set(CurrentScreenStatus(ScreenStatus::Unloading));
        *next_screen = NextScreen(Some(trigger.0));
        commands.run_schedule(UnloadSchedule::<S>::default());
        next_loading_state.set(ScreenLoadingState::<S>::Loading);
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

#[allow(unused_imports, reason = "used in docs")]
use bevy::app::FixedMain;
use bevy::{
    app::{FixedMainScheduleOrder, MainScheduleOrder},
    ecs::system::ScheduleSystem,
};

pub use crate::prelude::*;

mod data {
    use std::marker::PhantomData;

    use bevy::ecs::schedule::ScheduleLabel;

    use crate::prelude::*;

    /// Specifies the order of execution for a schedule.
    #[derive(Default, Debug)]
    pub enum Order {
        #[default]
        Before,
        After,
    }

    /// Manually triggered schedule which is called when the screen is unloaded.
    #[derive(ScheduleLabel, Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
    pub struct UnloadSchedule;

    /// Interal. Called during [UnloadSchedule]
    #[derive(SystemSet, Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
    pub struct UnloadSystems<S: Screen>(PhantomData<S>);

    /// Interal. Called after [UnloadSchedule]
    #[derive(SystemSet, Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
    pub struct PostUnloadSystems<S: Screen>(PhantomData<S>);
}
pub use data::Order;
use data::*;

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
            .add_systems(OnEnter(ScreenState::<S>::Ready), systems);
        self
    }

    pub fn on_unload<M>(self, systems: impl IntoScheduleConfigs<ScheduleSystem, M>) -> Self {
        self.app.init_schedule(UnloadSchedule);
        self.app.add_systems(
            UnloadSchedule,
            systems.in_set(UnloadSystems::<S>::default()),
        );
        self
    }

    /// Builds the schedule and adds it to the app.
    pub fn build(self) {
        let app = self.app;
        debug!("Building screen {:?}", S::name());

        // insert data
        app.add_schedule(self.schedule);
        app.init_resource::<S::SETTINGS>();
        let id = app.world_mut().register_system(S::spawn);
        let mut registry = app.world_mut().get_resource_or_init::<ScreenRegistry>();
        registry.insert(S::name(), id);

        // watch screen switcher
        app.add_observer(on_switch_screen::<S>);

        // scope systems
        let (config, fixed_config) = match S::STRATEGY {
            LoadingStrategy::Blocking => {
                let condition = in_state(ScreenState::<S>::Ready);
                (
                    self.scope.run_if(condition.clone()),
                    self.fixed_scope.run_if(condition),
                )
            }
            LoadingStrategy::Nonblocking => {
                let condition = not(in_state(ScreenState::<S>::Unloaded));
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

        // init state
        app.init_state::<ScreenState<S>>();
        app.add_loading_state(
            LoadingState::new(ScreenState::<S>::Loading)
                .continue_to_state(ScreenState::<S>::Ready)
                .load_collection::<S::ASSETS>(),
        );

        // set up unload schedule
        app.configure_sets(
            UnloadSchedule,
            (
                PostUnloadSystems::<S>::default().after(UnloadSystems::<S>::default()),
                UnloadSystems::<S>::default().run_if(not(in_state(ScreenState::<S>::Unloaded))),
                PostUnloadSystems::<S>::default().run_if(not(in_state(ScreenState::<S>::Unloaded))),
            ),
        );
        app.add_systems(
            UnloadSchedule,
            (
                unload::<S>.in_set(UnloadSystems::<S>::default()),
                post_unload::<S>.in_set(PostUnloadSystems::<S>::default()),
            ),
        );

        // Lifecycle
        app.add_systems(
            OnEnter(ScreenState::<S>::Loading),
            (|| debug!("Loading {:?}", S::name()), S::spawn),
        );
        app.add_systems(OnEnter(ScreenState::<S>::Ready), || {
            debug!("Ready {:?}", S::name())
        });
        app.add_systems(OnEnter(ScreenState::<S>::Unloading), || {
            debug!("Unloading {:?}", S::name())
        });
        app.add_systems(OnEnter(ScreenState::<S>::Unloaded), || {
            debug!("Unloaded {:?}", S::name())
        });
    }
}

fn on_switch_screen<T: Screen>(
    _trigger: On<SwitchToScreen<T>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<ScreenState<T>>>,
) {
    commands.run_schedule(UnloadSchedule);
    next_state.set(ScreenState::Loading);
}

fn unload<S: Screen>(mut next_state: ResMut<NextState<ScreenState<S>>>) {
    debug!("UnloadSystems {:?}", S::name());
    next_state.set(ScreenState::Unloading);
}

/// This function clears out all the non-screen-scoped entities.
fn post_unload<S: Screen>(
    mut commands: Commands,
    mut next_state: ResMut<NextState<ScreenState<S>>>,
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
) {
    debug!("PostUnloadSystems {:?}", S::name());
    screen_scoped.iter().for_each(|e| {
        commands.entity(e).despawn();
    });
    next_state.set(ScreenState::Unloaded);
}

use crate::prelude::*;
use bevy::ecs::schedule::ScheduleLabel;
use std::marker::PhantomData;

/// Component wrapper around a screen type.
#[derive(Deref, Component)]
#[component(on_add = T::init)]
pub struct ScreenWrapper<T: Screen>(pub T);

#[derive(Default, States, Debug, PartialEq, Eq, Reflect, Hash, Clone, Copy, Deref)]
pub struct CurrentScreen(pub Screens);
impl From<Screens> for CurrentScreen {
    fn from(value: Screens) -> Self {
        Self(value)
    }
}

#[derive(Default, States, Debug, PartialEq, Eq, Reflect, Hash, Clone, Copy, Deref)]
pub struct CurrentScreenStatus(pub ScreenStatus);
impl From<ScreenStatus> for CurrentScreenStatus {
    fn from(value: ScreenStatus) -> Self {
        Self(value)
    }
}

/// Stores next [Screens] state for unload logic.
#[derive(Resource, Default)]
pub struct NextScreen(pub Option<Screens>);

/// Triggered when a [Screen] finishes unloading and is
/// ready to transition.
#[derive(Event, Debug, PartialEq, Eq, Clone, Copy)]
pub struct FinishUnload;

/// Call this when you want to switch screens.
#[derive(Event, Debug, PartialEq, Eq, Clone, Copy, Deref)]
pub struct SwitchToScreen(pub Screens);

/// Enumerates possible screen states.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Reflect, Default)]
pub enum ScreenStatus {
    #[default]
    Loading,
    Ready,
    Unloading,
}

/// An empty asset collection. Indicates that a
/// screen should automatically transition from Loading to Ready.
#[derive(AssetCollection, Resource)]
pub struct EmptyAssetCollection {}

/// An empty settings parameter.
#[derive(Resource, Default)]
pub struct EmptySettings;

/// A screen's [Schedule]. All systems added to this schedule, using the
/// [ScreenScope] below, will be scoped to this screen's lifetime. That is,
/// they will only run when the screen is in [ScreenStatus::Ready].
#[allow(bevy::unconventional_naming)]
#[derive(ScheduleLabel, SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenScope<T: Screen> {
    _Ghost(PhantomData<T>),
    Update,
    FixedUpdate,
}

/// [LoadingState] for a [Screen]. See the [bevy_asset_loader] docs for more
/// info.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ScreenLoadingState<T: Screen> {
    #[default]
    Loading,
    Ready,
    _Phantom(PhantomData<T>),
}

/// Scopes an entity to the current screen. The entity will be cleaned up when
/// the [Screens] state changes. By default, all entities _except_ top-level
/// observers are automatically marked for cleanup.
///
/// Note that _children of this entity will not automatically be marked [Persistent]._
/// In order to propogate persistence, use the [Propogate] component.
#[derive(Component, Debug, Reflect, Clone, Copy, Default)]
pub struct ScreenScoped;

/// Marks an entity as screen-persistent, i.e., this entity will _not_ be
/// automatically cleaned up when the screen changes. By default, all entities
/// _except_ top-level [Observer]s and the [Window] are automtically marked for cleanup.
///
/// Note that _children of this entity will not automatically be marked [Persistent]._
/// In order to propogate persistence, use the [Propogate] component.
#[derive(Component, Debug, Reflect, Clone, Copy, Default)]
pub struct Persistent;

use std::marker::PhantomData;

use crate::prelude::*;
use bevy::ecs::schedule::ScheduleLabel;

/// Scopes an entity to the current screen. The entity will be cleaned up when
/// the [Screens] state changes. By default, all entities _except_ top-level
/// observers are automatically marked for cleanup.
#[derive(Component, Debug, Reflect)]
pub struct ScreenScoped;

/// Marks an entity as screen-persistent, i.e., this entity will _not_ be
/// automatically cleaned up when the screen changes. By default, all entities
/// _except_ top-level observers are automtically marked for cleanup.
#[derive(Component, Debug, Reflect)]
pub struct Persistent;

#[derive(Default, States, Debug, PartialEq, Eq, Reflect, Hash, Clone, Copy)]
pub struct CurrentScreen {
    pub screen: Screens,
    pub status: ScreenStatus,
}
impl CurrentScreen {
    pub fn new<T: Screen>() -> Self {
        Self {
            screen: T::NAME,
            status: ScreenStatus::Ready,
        }
    }
    pub fn loading(self) -> Self {
        Self {
            status: ScreenStatus::Loading,
            ..self
        }
    }
    pub fn ready(self) -> Self {
        Self {
            status: ScreenStatus::Ready,
            ..self
        }
    }
    pub fn unloading(self) -> Self {
        Self {
            status: ScreenStatus::Unloading,
            ..self
        }
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
#[derive(Event, Deref, Debug, PartialEq, Eq, Clone)]
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
#[derive(ScheduleLabel, SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenScope<T: Screen> {
    _Ghost(PhantomData<T>),
    Update,
    FixedUpdate,
}

use std::marker::PhantomData;

use crate::prelude::*;
use bevy::ecs::{
    schedule::{IntoScheduleConfigs, ScheduleConfigs, ScheduleLabel},
    system::ScheduleSystem,
};

/// Helper struct for systems associated with a service.
/// Allows for easy system scoping.
#[derive(Deref, DerefMut)]
pub struct ServiceSystems(pub(crate) ScheduleConfigs<ScheduleSystem>);
impl ServiceSystems {
    pub fn new<M>(systems: impl IntoScheduleConfigs<ScheduleSystem, M>) -> Self {
        Self(systems.into_configs())
    }
    pub fn take(self) -> ScheduleConfigs<ScheduleSystem> {
        self.0
    }
}

/// Scopes an entity to the current screen.
/// The entity will be cleaned up when the [Screens] state changes.
#[derive(Component, Debug, Reflect)]
pub struct ScreenScoped;

#[derive(States, Resource, Debug, PartialEq, Eq, Reflect, Hash, Clone, Copy)]
pub struct CurrentScreen {
    pub screen: Screens,
    pub status: ScreenStatus,
}
impl CurrentScreen {
    pub fn new<S: Screen>() -> Self {
        Self {
            screen: S::NAME,
            status: ScreenStatus::Ready,
        }
    }
    pub fn loading(self) -> Self {
        Self {
            screen: self.screen,
            status: ScreenStatus::Loading,
        }
    }
    pub fn ready(self) -> Self {
        Self {
            screen: self.screen,
            status: ScreenStatus::Ready,
        }
    }
    pub fn unloading(self) -> Self {
        Self {
            screen: self.screen,
            status: ScreenStatus::Unloading,
        }
    }
}

/// Stores next [Screens] state for unload logic.
#[derive(Resource)]
pub struct NextScreen(pub Option<Screens>);

/// Triggered when a [Screen] finishes unloading and is
/// ready to transition.
#[derive(Event, Debug, PartialEq, Eq, Clone, Copy)]
pub struct FinishUnload;

/// Call this when you want to switch screens.
#[derive(Event, Deref, Debug, PartialEq, Eq, Clone, Copy)]
pub struct SwitchScreen(pub Screens);

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

/// A screen's [Schedule]. All systems added to this schedule, using the
/// [ScreenScope] below, will be scoped to this screen's lifetime. That is,
/// they will only run when the screen is in [ScreenStatus::Ready].
#[derive(ScheduleLabel, SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenScope<T: Screen> {
    _Ghost(PhantomData<T>),
    Update,
    FixedUpdate,
}

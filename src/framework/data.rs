use crate::prelude::*;
use bevy::{
    ecs::{
        schedule::{IntoScheduleConfigs, ScheduleConfigs},
        system::ScheduleSystem,
    },
    reflect::{DynamicEnum, DynamicTuple, DynamicVariant},
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

/// Stores next [Screens] state for unload logic.
#[derive(Resource)]
pub struct NextScreen(pub Option<Screens>);

/// Triggered when a [Screen] finishes unloading and is
/// ready to transition.
#[derive(Event)]
pub struct FinishUnload;

#[derive(Event)]
pub struct SwitchScreen(Screens);
impl SwitchScreen {
    /// Variant name _must_ be an exact match.
    // TODO: Is there a better way?
    pub fn new(variant: &str) -> Self {
        let mut tup = DynamicTuple::default();
        tup.insert(ScreenStatus::Loading);
        let dy = DynamicEnum::new(variant, DynamicVariant::Tuple(tup));
        Self(Screens::from_reflect(&dy).unwrap())
    }

    pub fn screen(&self) -> Screens {
        self.0
    }
}

/// Enumerates possible screen states.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Reflect, Default)]
pub enum ScreenStatus {
    #[default]
    Uninitialized,
    Loading,
    Ready,
    Unloading,
}

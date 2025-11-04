use crate::prelude::*;
use bevy::ecs::{
    schedule::{IntoScheduleConfigs, ScheduleConfigs},
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

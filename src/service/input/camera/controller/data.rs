use crate::prelude::*;

#[derive(Debug, Reflect, Copy, Clone)]
pub enum CameraControllerKind {
    Fly,
    Tracking,
}

#[derive(Debug, Reflect, Component, Copy, Clone)]
#[require(Camera {is_active: false, ..Default::default()}, Camera3d)]
#[component(immutable)]
pub struct CameraController {
    /// Is the controller enabled?
    pub enabled: bool,
    /// Is the camera active?
    pub active: bool,
    /// What kind of camera is this?
    pub kind: CameraControllerKind,
}
impl CameraController {
    pub fn new(kind: CameraControllerKind) -> Self {
        Self {
            enabled: true,
            active: true,
            kind,
        }
    }
    pub fn with_enabled(self, enabled: bool) -> Self {
        Self { enabled, ..self }
    }
    pub fn with_active(self, active: bool) -> Self {
        Self { active, ..self }
    }
}

/// Used to update the camera controller.
#[derive(Event, Debug)]
pub struct InsertCameraController {
    pub entity: Entity,
    pub new_controller: CameraController,
}

use crate::prelude::*;

#[derive(Component)]
#[require(CameraController::new(CameraControllerKind::Fly))]
pub struct FlyCam;

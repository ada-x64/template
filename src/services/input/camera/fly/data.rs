use crate::prelude::*;

#[derive(Component)]
#[require(CameraController::new(CameraControllerKind::Fly), ICtxFlyCam)]
pub struct FlyCam;

#[derive(Component, Default)]
pub struct ICtxFlyCam;

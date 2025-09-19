// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub const PLAYER_CAPSULE_HEIGHT: f32 = 3.;
pub const PLAYER_CAPSULE_RADIUS: f32 = 0.5;
pub const PLAYER_DEFAULT_SPEED: f32 = 10.;
pub const PLAYER_CAM_ROTATION_SPD: f32 = 10.;
pub const PLAYER_CAM_ZOOM_SPD: f32 = 10.;

#[derive(SystemSet, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlayerSystems;

#[derive(Event, Reflect, Copy, Clone, Debug)]
pub struct SpawnPlayerRoot;

#[derive(Component, Debug, Default)]
#[require(Name::new("PlayerCam"))]
pub struct PlayerCam;

#[derive(Component, Default)]
#[require(Name::new("PlayerController"))]
pub struct PlayerController {
    pub last_move: Option<Vec3>,
}

#[derive(Component, Debug)]
#[require(Name::new("PlayerCamController"))]
pub struct PlayerCamController {
    /// In radians.
    pub rotation: Vec2,
    /// percentage zoomed out (e.g. value of 1 means outer_radius is at 100% its default length)
    pub zoom: f32,
    /// radius of outer sphere. used for zoom and camera collisions.
    pub outer_radius: f32,
    /// radius of inner sphere. used for zoom and camera collisions.
    pub inner_radius: f32,
    /// Desired translation.
    pub desired_tl: Vec3,
}
impl PlayerCamController {
    pub fn new(desired_tl: Vec3) -> Self {
        Self {
            rotation: Vec2::ZERO,
            zoom: 1.,
            outer_radius: 10.,
            inner_radius: 1.,
            desired_tl,
        }
    }
}
/// Default player input context
#[derive(Component)]
pub struct ICtxDefault;

/// Default camera input context
#[derive(Component)]
pub struct ICtxCamDefault;

/// PlayerAction_Move
#[derive(InputAction, Reflect)]
#[action_output(Vec3)]
pub struct PAMove;

/// PlayerAction_RotateCamera
#[derive(InputAction, Reflect)]
#[action_output(Vec2)]
pub struct PARotateCam;

/// PlayerAction_ZoomCamera
#[derive(InputAction, Reflect)]
#[action_output(f32)]
pub struct PAZoomCam;

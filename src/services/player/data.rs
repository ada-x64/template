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
pub struct PlayerCam;

#[derive(Component, Default)]
pub struct PlayerController {
    pub last_move: Option<Vec3>,
}

#[derive(Component)]
pub struct PlayerCamController {
    pub rotation: f32, // radians
    pub zoom: f32,     // percentage
    pub max_zoom: f32,
    pub min_zoom: f32,
}
impl Default for PlayerCamController {
    fn default() -> Self {
        Self {
            rotation: 0.,
            zoom: 1.,
            max_zoom: 20.,
            min_zoom: 0.1,
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

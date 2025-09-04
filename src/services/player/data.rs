// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub const PLAYER_CAPSULE_HEIGHT: f32 = 3.;
pub const PLAYER_CAPSULE_RADIUS: f32 = 0.5;
pub const PLAYER_DEFAULT_SPEED: f32 = 10.;

#[derive(SystemSet, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlayerSystems;

#[derive(Event, Reflect, Copy, Clone, Debug)]
pub struct SpawnPlayerRoot;

#[derive(Component, Debug, Default)]
pub struct PlayerCam;

#[derive(Component)]
pub struct PlayerRoot;

#[derive(Component, Default)]
pub struct PlayerController {
    pub move_vec: Vec3,
}

/// Default input context
#[derive(Component)]
pub struct ICtxDefault;

/// PlayerAction_Move
#[derive(InputAction, Reflect)]
#[action_output(Vec3)]
pub struct PAMove;

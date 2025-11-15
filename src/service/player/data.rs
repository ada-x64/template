use crate::prelude::*;

pub const PLAYER_CAPSULE_HEIGHT: f32 = 3.;
pub const PLAYER_CAPSULE_RADIUS: f32 = 0.5;
pub const PLAYER_DEFAULT_SPEED: f32 = 10.;
pub const PLAYER_CAM_ROTATION_SPD: f32 = 10.;
pub const PLAYER_CAM_ZOOM_SPD: f32 = 10.;

#[derive(SystemSet, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlayerSystems;

#[derive(Event, Reflect, Copy, Clone, Debug)]
pub struct SpawnPlayerRoot;

#[derive(Component, Default)]
#[require(Name::new("Player Controller"))]
pub struct PlayerController {
    pub last_move: Option<Vec3>,
}

/// Default player input context
#[derive(Component)]
pub struct ICtxDefault;

/// PlayerAction_Move
#[derive(InputAction, Reflect)]
#[action_output(Vec3)]
pub struct PAMove;

#[derive(AssetCollection, Resource, Default, Debug)]
pub struct PlayerAssets {
    #[asset(path = "models/basil.glb#Scene0")]
    pub model: Handle<Scene>,
}

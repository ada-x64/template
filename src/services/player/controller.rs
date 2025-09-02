//! Contains the controller for the player.

use avian3d::prelude::*;
pub use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;

use crate::services::player::{
    assets::PlayerAssets,
    data::{PLAYER_CAPSULE_HEIGHT, PLAYER_CAPSULE_RADIUS},
};

/// Constructor for the player controller bundle
pub fn player_controller() -> impl Bundle {
    (
        Name::new("Player Controller"),
        RigidBody::Dynamic,
        Collider::capsule(PLAYER_CAPSULE_RADIUS, PLAYER_CAPSULE_HEIGHT),
        TnuaController::default().basis(TnuaBuiltinWalk::default()),
        TnuaAvian3dSensorShape(Collider::cylinder(PLAYER_CAPSULE_RADIUS - 0.1, 0.0)),
        LockedAxes::ROTATION_LOCKED,
        Friction::ZERO,
    )
}

/// Constructor for the player mesh bundle
pub fn spawn_player_mesh(
    player_assets: Res<PlayerAssets>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    scene_spawner.spawn(player_assets.model.clone());
}

// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
pub(crate) mod assets;
pub(crate) mod camera;
pub(crate) mod controller;
pub(crate) mod data;

use crate::data::*;
use avian3d::prelude::*;
use bevy::{prelude::*, render::view::RenderLayers};
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use data::*;

use crate::{
    screens::ScreenStates,
    services::player::{assets::PlayerAssets, data::SpawnPlayerRoot},
};

pub fn spawn_player_root(
    _: Trigger<SpawnPlayerRoot>,
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
) {
    commands.spawn((
        Name::new("PlayerRoot"),
        PlayerRoot,
        Visibility::Visible,
        StateScoped(ScreenStates::InWorld),
        Transform::from_xyz(0., 10., 0.), // TODO: Should be set relative to terrain
        children![
            (
                Name::new("Player Controller"),
                PlayerController::default(),
                Transform::IDENTITY,
                RigidBody::Dynamic,
                Collider::capsule(PLAYER_CAPSULE_RADIUS, PLAYER_CAPSULE_HEIGHT),
                TnuaController::default(),
                TnuaAvian3dSensorShape(Collider::cylinder(PLAYER_CAPSULE_RADIUS - 0.1, 0.0)),
                LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
                Friction::ZERO,
                SceneRoot(player_assets.model.clone()),
                ICtxDefault,
            ),
            (
                Name::new("PlayerCam"),
                PlayerCam,
                StateScoped(ScreenStates::InWorld),
                Transform::IDENTITY,
                Camera3d::default(),
                PointLight::default(),
                #[cfg(feature = "dev")]
                ShowLightGizmo::default(),
                Camera {
                    order: CameraOrder::World.into(),
                    ..Default::default()
                },
                RenderLayers::from(
                    RenderLayer::DEFAULT | RenderLayer::GIZMOS_3D | RenderLayer::PARTICLES
                ),
            )
        ],
    ));
}

pub fn plugin(app: &mut App) {
    app.add_plugins(controller::plugin)
        .add_systems(FixedUpdate, (camera::track_player).in_set(PlayerSystems))
        .add_observer(spawn_player_root)
        .configure_sets(
            FixedUpdate,
            PlayerSystems.run_if(in_state(ScreenStates::InWorld)),
        );
}

// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
pub(crate) mod assets;
pub(crate) mod controller;
pub(crate) mod data;

use crate::{
    prelude::*,
    services::{input::camera::tracking::tracking_cam_bundle, player::assets::PlayerAssets},
};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::ContextActivity;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use data::*;

pub mod prelude {
    pub use super::data::*;
}

fn spawn_player_root(
    _: Trigger<SpawnPlayerRoot>,
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut camera_list: ResMut<CameraList>,
) {
    let player_entt = commands
        .spawn((
            PlayerController::default(),
            StateScoped(ScreenStates::InWorld),
            SceneRoot(player_assets.model.clone()),
            (
                RigidBody::Dynamic,
                Collider::capsule(PLAYER_CAPSULE_RADIUS, PLAYER_CAPSULE_HEIGHT),
                LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
                Friction::ZERO,
            ),
            (
                TnuaController::default(),
                TnuaAvian3dSensorShape(Collider::cylinder(PLAYER_CAPSULE_RADIUS + 0.1, 0.)),
                ICtxDefault,
                ContextActivity::<ICtxDefault>::ACTIVE,
            ),
        ))
        .id();

    let cam = commands
        .spawn((
            Name::new("PlayerCam"),
            StateScoped(ScreenStates::InWorld),
            (LockedAxes::new().lock_rotation_z(),),
            (
                #[cfg(feature = "dev")]
                ShowLightGizmo::default(),
                PointLight::default(),
            ),
            tracking_cam_bundle(player_entt, Vec3::new(0., 1., 1.)),
        ))
        .id();
    camera_list.push(cam);
}

pub fn plugin(app: &mut App) {
    app.add_plugins(controller::plugin)
        .add_observer(spawn_player_root)
        .configure_sets(
            FixedUpdate,
            PlayerSystems.run_if(in_state(ScreenStates::InWorld)),
        );
}

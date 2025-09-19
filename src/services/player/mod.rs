// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
pub(crate) mod assets;
pub(crate) mod camera;
pub(crate) mod controller;
pub(crate) mod data;

use crate::{data::*, services::data::CollisionLayer};
use avian3d::prelude::*;
use bevy::{prelude::*, render::view::RenderLayers};
use bevy_enhanced_input::prelude::ContextActivity;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use data::*;

use crate::{
    screens::ScreenStates,
    services::player::{assets::PlayerAssets, data::SpawnPlayerRoot},
};

pub mod prelude {
    pub use super::data::*;
}

fn spawn_player_root(
    _: Trigger<SpawnPlayerRoot>,
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
) {
    // TODO: Should be from terrain height.
    let player_tl = Vec3::new(0., 10., 0.);
    let cam_tl = player_tl + Vec3::new(0., 5., 5.);
    let player_tf = Transform::from_translation(player_tl);
    let cam_tf = Transform::from_translation(cam_tl);
    commands.spawn((
        PlayerController::default(),
        StateScoped(ScreenStates::InWorld),
        RigidBody::Dynamic,
        Collider::capsule(PLAYER_CAPSULE_RADIUS, PLAYER_CAPSULE_HEIGHT),
        TnuaController::default(),
        TnuaAvian3dSensorShape(Collider::cylinder(PLAYER_CAPSULE_RADIUS + 0.1, 0.)),
        LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
        Friction::ZERO,
        ICtxDefault,
        ContextActivity::<ICtxDefault>::ACTIVE,
        SceneRoot(player_assets.model.clone()),
    ));
    commands.spawn((RayCaster::new(
        player_tf.translation,
        Dir3::new((cam_tl - player_tl).normalize()).unwrap(),
    )
    .with_query_filter(SpatialQueryFilter::from_mask(
        CollisionLayer::Camera | CollisionLayer::Default,
    ))
    .with_max_hits(1),));
    commands.spawn((
        Name::new("PlayerCamRoot"),
        (
            PlayerCamController::new(cam_tl),
            cam_tf,
            ICtxCamDefault,
            ContextActivity::<ICtxCamDefault>::ACTIVE,
            LockedAxes::new().lock_rotation_z(),
        ),
        (
            PlayerCam,
            StateScoped(ScreenStates::InWorld),
            Camera3d::default(),
            #[cfg(feature = "dev")]
            ShowLightGizmo::default(),
            PointLight::default(),
            Camera {
                is_active: true,
                order: CameraOrder::World.into(),
                ..Default::default()
            },
            RenderLayers::from(
                RenderLayer::DEFAULT | RenderLayer::GIZMOS_3D | RenderLayer::PARTICLES,
            ),
        ),
    ));
}

pub fn plugin(app: &mut App) {
    app.add_plugins((controller::plugin, camera::plugin))
        .add_observer(spawn_player_root)
        .configure_sets(
            FixedUpdate,
            PlayerSystems.run_if(in_state(ScreenStates::InWorld)),
        );
}

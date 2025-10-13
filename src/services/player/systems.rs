// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use crate::prelude::*;

fn update_controller(
    mut query: Single<(&mut TnuaController, &mut PlayerController)>,
    cam_tf: Single<&Transform, With<TrackingCam>>,
) {
    let (tnua, controller) = &mut *query;

    let yaw = cam_tf.rotation.to_euler(EulerRot::YXZ).0;
    let yaw_quat = Quat::from_axis_angle(Vec3::Y, yaw);
    let moved = controller.last_move.is_some();
    let last_move = controller.last_move.take().unwrap_or_default();
    let desired_velocity = yaw_quat * last_move;
    let desired_forward = moved
        .then_some(Dir3::new(-desired_velocity.normalize()).ok())
        .flatten();

    tnua.basis(TnuaBuiltinWalk {
        desired_velocity: yaw_quat * last_move,
        float_height: PLAYER_CAPSULE_HEIGHT / 2. + PLAYER_CAPSULE_RADIUS,
        desired_forward,
        turning_angvel: 10000.,
        ..Default::default()
    });
}

pub fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, (update_controller).in_set(PlayerSystems));
}

// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_tnua::prelude::*;

use crate::services::player::data::*;

pub fn spawn_player_actions(event: Trigger<OnAdd, ICtxDefault>, mut commands: Commands) {
    commands.entity(event.target()).insert(actions!(
        ICtxDefault[(
            Action::<PAMove>::new(),
            DeadZone::default(),
            SmoothNudge::default(),
            Scale::splat(PLAYER_DEFAULT_SPEED),
            Negate::y(),
            SwizzleAxis::XZY,
            Bindings::spawn((Cardinal::wasd_keys(), Axial::left_stick())),
        )]
    ));
}

// // Observers
pub fn on_move(trigger: Trigger<Fired<PAMove>>, mut controller: Single<&mut PlayerController>) {
    controller.last_move = Some(trigger.value);
}

pub fn update_controller(
    mut query: Single<(&mut TnuaController, &mut PlayerController)>,
    cam_tf: Single<&Transform, With<PlayerCam>>,
) {
    let (tnua, controller) = &mut *query;

    let yaw = cam_tf.rotation.to_euler(EulerRot::YXZ).0;
    let yaw_quat = Quat::from_axis_angle(Vec3::Y, yaw);
    let last_move = controller.last_move.take().unwrap_or_default();

    tnua.basis(TnuaBuiltinWalk {
        desired_velocity: yaw_quat * last_move,
        float_height: PLAYER_CAPSULE_HEIGHT / 2. + 0.01,
        ..Default::default()
    })
}

// Plugin
pub fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, (update_controller).in_set(PlayerSystems))
        .add_observer(spawn_player_actions)
        .add_observer(on_move)
        .add_input_context::<ICtxDefault>();
}

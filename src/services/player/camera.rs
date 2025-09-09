// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
#[cfg(feature = "dev")]
use bevy_simple_subsecond_system::hot;

use crate::services::{data::GrabCursor, player::data::*};

fn spawn_cam_actions(event: Trigger<OnAdd, ICtxCamDefault>, mut commands: Commands) {
    info!("spawn_cam_actions");
    commands.entity(event.target()).insert(actions![
        ICtxCamDefault[
        (
            Action::<PARotateCam>::new(),
            Bindings::spawn((
                Axial::right_stick().with((Scale::splat(2.0), Negate::x())),
                Spawn((Binding::mouse_motion(), Scale::splat(0.1), Negate::all()))
            )),
        ),
        (
            Action::<PAZoomCam>::new(),
            DeadZone::default(),
            SmoothNudge::default(),
            Scale::splat(PLAYER_CAM_ZOOM_SPD),
            Bindings::spawn(
                (
                    Axial::right_stick(),
                    Spawn(
                        (Binding::mouse_wheel(), SwizzleAxis::YXZ))
                )
            )
        ),
        (Action::<PACaptureCursor>::new(), bindings![MouseButton::Left]),
        (Action::<PAReleaseCursor>::new(), bindings![KeyCode::Escape]),
        ]
    ]);
}

/// Spawns the gameplay camera
fn track_player(
    pt: Single<&Transform, (With<PlayerController>, Without<PlayerCam>)>,
    mut ct: Single<&mut Transform, (With<PlayerCam>, Without<PlayerRoot>)>,
) {
    **ct = ct
        .looking_at(pt.translation, Vec3::Y)
        .with_translation(pt.translation - Vec3::new(0., -5., -10.));
}

// observers
fn on_rotate(
    trigger: Trigger<Fired<PARotateCam>>,
    mut controller: Query<&mut PlayerCamController>,
) {
    info_once!("Got rotatation trigger! value={}", trigger.value);
    controller.get_mut(trigger.target()).unwrap().rotate =
        Some(Quat::from_axis_angle(Vec3::Y, trigger.value.x));
}

fn on_zoom(trigger: Trigger<Fired<PAZoomCam>>, mut controller: Query<&mut PlayerCamController>) {
    info_once!("Got zoom trigger! value={}", trigger.value);
    controller.get_mut(trigger.target()).unwrap().zoom = Some(trigger.value);
}

fn on_capture_cursor(_trigger: Trigger<Fired<PACaptureCursor>>, mut commands: Commands) {
    info!("Capturing cursor!");
    commands.send_event(GrabCursor::<true>);
}

fn on_release_cursor(_trigger: Trigger<Fired<PAReleaseCursor>>, mut commands: Commands) {
    info!("Releasing cursor!");
    commands.send_event(GrabCursor::<false>);
}

#[cfg_attr(feature = "dev", hot)]
fn camera_controls(
    pt: Single<&Transform, (With<PlayerController>, Without<PlayerCam>)>,
    mut ct: Single<(&mut Transform, &mut PlayerCamController), Without<PlayerController>>,
) {
    let (ref mut ct, ref mut controller) = *ct;
    ct.translate_around(Vec3::ZERO, controller.rotate.take().unwrap_or_default());
    ct.look_at(pt.translation, Vec3::Y);
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (track_player, camera_controls)
            .chain()
            .in_set(PlayerSystems),
    )
    .add_input_context::<ICtxCamDefault>()
    .add_observer(spawn_cam_actions)
    .add_observer(on_zoom)
    .add_observer(on_rotate)
    .add_observer(on_capture_cursor)
    .add_observer(on_release_cursor);
}

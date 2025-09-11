use avian3d::math::PI;

// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use crate::prelude::*;

fn spawn_cam_actions(event: Trigger<OnAdd, ICtxCamDefault>, mut commands: Commands) {
    info!("spawn_cam_actions");
    commands.entity(event.target()).insert(actions![
        ICtxCamDefault[
        (
            Action::<PARotateCam>::new(),
            Bindings::spawn((
                Axial::right_stick().with((Scale::splat(2.0), Negate::x())),
                Spawn((Binding::mouse_motion(), Scale::splat(0.01), Negate::all()))
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
                        (Binding::mouse_wheel(), Scale::splat(0.1), SwizzleAxis::YXZ))
                )
            )
        ),
        ]
    ]);
}

// observers
fn on_rotate(
    trigger: Trigger<Fired<PARotateCam>>,
    mut controller: Query<&mut PlayerCamController>,
) {
    debug!("Got rotatation trigger! value={}", trigger.value);
    let mut controller = controller.get_mut(trigger.target()).unwrap();
    controller.rotation = (controller.rotation + trigger.value.x) % (2. * PI);
}

fn on_zoom(trigger: Trigger<Fired<PAZoomCam>>, mut controller: Query<&mut PlayerCamController>) {
    debug!("Got zoom trigger! value={}", trigger.value);
    let mut controller = controller.get_mut(trigger.target()).unwrap();
    controller.zoom = f32::clamp(
        controller.zoom + trigger.value,
        controller.min_zoom,
        controller.max_zoom,
    );
}

#[cfg_attr(feature = "dev", hot)]
fn camera_controls(
    pt: Single<&Transform, (With<PlayerController>, Without<PlayerCam>)>,
    mut ct: Single<(&mut Transform, &mut PlayerCamController), Without<PlayerController>>,
    window: Single<&Window>,
) {
    use bevy::window::CursorGrabMode;
    // do this, but also disable ctx when flycam is enabled
    if window.cursor_options.grab_mode != CursorGrabMode::Locked {
        return;
    }
    let (ref mut ct, ref mut controller) = *ct;
    **ct = **pt;
    ct.rotation = Quat::from_axis_angle(Vec3::Y, controller.rotation);
    ct.translation = pt.translation + ct.rotation * (Vec3::new(0., 5., 10.) / controller.zoom);
    ct.look_at(pt.translation, Vec3::Y);
}

pub fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, (camera_controls).chain().in_set(PlayerSystems))
        .add_input_context::<ICtxCamDefault>()
        .add_observer(spawn_cam_actions)
        .add_observer(on_zoom)
        .add_observer(on_rotate);
}

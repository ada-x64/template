// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use crate::{prelude::*, services::data::CollisionLayer};
use avian3d::{math::PI, prelude::*};

// The idea:
// Minimum and maximum distance spheres.
// The camera tries to maintain the maximum distance,
// but collisions with physics objects will move the camera towards the minimum distance sphere.
// The camera _cannot_ clip through the minimum distance sphere and will maintain at least that much distance.
// Zoom = changing outer sphere radius.
// No need for colliders, could just cast a single ray from the player to the desired camera position.
// If there is a collision, then (smoothly) move to the collisions location.

fn spawn_cam_actions(event: Trigger<OnAdd, ICtxCamDefault>, mut commands: Commands) {
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
    let mut controller = controller.get_mut(trigger.target()).unwrap();
    controller.rotation.x = (controller.rotation.x + trigger.value.x) % (2. * PI);
    controller.rotation.y = (controller.rotation.y + trigger.value.y).clamp(-PI / 8., 1. * PI / 8.);
}

fn on_zoom(trigger: Trigger<Fired<PAZoomCam>>, mut controller: Query<&mut PlayerCamController>) {
    let mut controller = controller.get_mut(trigger.target()).unwrap();
    controller.zoom = f32::clamp(controller.zoom + trigger.value, 0., 1.);
}

#[cfg_attr(feature = "dev", hot)]
fn apply(
    pt: Single<&Transform, (With<PlayerController>, Without<PlayerCam>)>,
    mut ct: Single<(&mut Transform, &mut PlayerCamController), Without<PlayerController>>,
    mut caster_q: Single<(&mut RayCaster, &RayHits)>,
    window: Single<&Window>,
) {
    use bevy::window::CursorGrabMode;
    // do this, but also disable ctx when flycam is enabled
    if window.cursor_options.grab_mode != CursorGrabMode::Locked {
        return;
    }
    let (ref mut caster, hits) = *caster_q;
    let (ref mut ct, ref mut controller) = *ct;

    // set desired position
    **ct = **pt;
    let rotation = Quat::from_axis_angle(Vec3::Y, controller.rotation.x)
        * Quat::from_axis_angle(Vec3::X, controller.rotation.y);
    let max_dist = hits
        .as_slice()
        .first()
        .map(|d| d.distance)
        .unwrap_or(caster.max_distance);
    ct.translation = pt.translation + rotation * (Vec3::new(0., 5., 5.) * max_dist);
    ct.look_at(pt.translation, Vec3::Y);

    // set up ray for next pass
    caster.origin = pt.translation;
    let dir = rotation * Vec3::ONE;
    caster.direction = Dir3::new(dir).unwrap();
    caster.max_distance = controller.outer_radius * controller.zoom;
}

pub fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, (apply).in_set(PlayerSystems))
        .add_input_context::<ICtxCamDefault>()
        .add_observer(spawn_cam_actions)
        .add_observer(on_zoom)
        .add_observer(on_rotate);
}

use std::f32::consts::{FRAC_PI_2, FRAC_PI_8};

// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use super::data::*;
use crate::prelude::*;
use avian3d::{math::PI, prelude::*};
use bevy::render::view::RenderLayers;
use tiny_bail::prelude::*;

// The idea:
// Minimum and maximum distance spheres.
// The camera tries to maintain the maximum distance,
// but collisions with physics objects will move the camera towards the minimum distance sphere.
// The camera _cannot_ clip through the minimum distance sphere and will maintain at least that much distance.
// Zoom = changing outer sphere radius.
// No need for colliders, could just cast a single ray from the player to the desired camera position.
// If there is a collision, then (smoothly) move to the collisions location.

pub fn tracking_cam_bundle(tracking: Entity, offset: Vec3) -> impl Bundle {
    (
        TrackingCam::new(tracking),
        // rendering
        (
            Camera {
                order: CameraOrder::World as isize,
                ..Default::default()
            },
            RenderLayers::from(
                RenderLayer::DEFAULT | RenderLayer::GIZMOS_3D | RenderLayer::PARTICLES,
            ),
        ),
        // physics
        (
            RayCaster::new(Vec3::ZERO, Dir3::new(offset.normalize()).unwrap())
                .with_query_filter(SpatialQueryFilter::from_mask(
                    CollisionLayer::Camera | CollisionLayer::Default,
                ))
                .with_max_hits(1),
        ),
        // actions
        (
            ContextActivity::<ICtxTrackingCam>::ACTIVE,
            ICtxTrackingCam,
            actions![
                ICtxTrackingCam[
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
            ],
        ),
    )
}

fn on_rotate(trigger: Trigger<Fired<PARotateCam>>, mut controller: Query<&mut TrackingCam>) {
    let mut controller = rq!(controller.get_mut(trigger.target()));
    controller.rotation.x = (controller.rotation.x + trigger.value.x) % (2. * PI);
    controller.rotation.y = (controller.rotation.y + trigger.value.y).clamp(-FRAC_PI_8, FRAC_PI_8);
}

fn on_zoom(trigger: Trigger<Fired<PAZoomCam>>, mut projections: Query<&mut Projection>) {
    let mut projection = projections.get_mut(trigger.target()).unwrap();
    let Projection::Perspective(projection) = &mut *projection else {
        panic!("camera should be perspective");
    };
    projection.fov = (projection.fov + trigger.value).clamp(FRAC_PI_8, FRAC_PI_2);
}

fn apply(
    tracked_tf: Query<&Transform, Without<TrackingCam>>,
    mut cam_tf: Single<(&mut Transform, &mut TrackingCam), Without<PlayerController>>,
    mut caster_q: Single<(&mut RayCaster, &RayHits)>,
    window: Single<&Window>,
) {
    use bevy::window::CursorGrabMode;
    // do this, but also disable ctx when flycam is enabled
    if window.cursor_options.grab_mode != CursorGrabMode::Locked {
        return;
    }
    let (ref mut caster, hits) = *caster_q;
    let (ref mut cam_tf, ref mut controller) = *cam_tf;
    let tracked_tf = r!(tracked_tf.get(controller.entity));

    // set desired transform
    let rotation = Quat::from_axis_angle(Vec3::Y, controller.rotation.x)
        * Quat::from_axis_angle(Vec3::X, controller.rotation.y);
    let max_dist = hits
        .as_slice()
        .first()
        .map(|d| d.distance)
        .unwrap_or(caster.max_distance);
    cam_tf.translation = tracked_tf.translation + rotation * (Vec3::new(0., 1., 1.) * max_dist);
    cam_tf.look_at(tracked_tf.translation, Vec3::Y);

    // set up ray for next pass
    caster.origin = tracked_tf.translation;
    let dir = rotation * Vec3::ONE;
    caster.direction = Dir3::new(dir).unwrap();
}

pub fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, (apply).in_set(PlayerSystems))
        .add_input_context::<ICtxTrackingCam>()
        .add_observer(on_zoom)
        .add_observer(on_rotate);
}

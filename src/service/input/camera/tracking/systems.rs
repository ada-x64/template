use crate::prelude::*;
use bevy::window::CursorGrabMode;

fn apply(
    tracked_tf: Query<&Transform, Without<TrackingCam>>,
    mut cam_tf: Single<(&mut Transform, &mut TrackingCam), Without<PlayerController>>,
    mut caster_q: Single<(&mut RayCaster, &RayHits)>,
    window: Single<&Window>,
) {
    // TODO relationship
    // commands.entity(tracking).add_child((
    //     TrackingCamRayCast,
    //     RayCaster::new(Vec3::ZERO, Dir3::new(offset.normalize()).unwrap())
    //         .with_query_filter(SpatialQueryFilter::from_mask(
    //             CollisionLayer::Camera | CollisionLayer::Default,
    //         ))
    //         .with_max_hits(1)
    //         .with_solidness(true),
    // ));
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
        .map(|d| d.distance.min(controller.outer_radius))
        .unwrap_or(controller.outer_radius);
    cam_tf.translation = tracked_tf.translation + rotation * (Vec3::new(0., 1., 1.) * max_dist);
    cam_tf.look_at(tracked_tf.translation, Vec3::Y);

    // set up ray for next pass
    caster.origin = tracked_tf.translation;
    caster.direction = Dir3::new(rotation * Vec3::NEG_ONE).unwrap();
}

pub fn systems() -> ServiceSystems {
    ServiceSystems::new(apply)
}

use crate::prelude::*;

/// Tracking camera. Will follow the given entity. Will spawn a CameraController on add.
/// Prefer to use [tracking_cam_bundle].
#[derive(Component, Debug)]
#[require(CameraController::new(CameraControllerKind::Tracking), ICtxTrackingCam)]
pub struct TrackingCam {
    /// In radians.
    pub rotation: Vec2,
    /// radius of outer sphere. used for zoom and camera collisions.
    pub outer_radius: f32,
    /// Tracking entity.
    pub entity: Entity,
}
impl TrackingCam {
    pub fn new(entity: Entity) -> Self {
        Self {
            rotation: Vec2::ZERO,
            outer_radius: 10.,
            entity,
        }
    }
}

#[derive(Component)]
#[relationship(relationship_target = Tracking)]
pub struct TrackedBy {
    #[relationship]
    tracker: Entity,
}

#[derive(Component)]
#[relationship_target(relationship = TrackedBy)]
pub struct Tracking(Entity);

/// Spawned as a child of the tracked entity.
/// Contains a reference to the tracking cam.
#[derive(Component)]
pub struct TrackingCamRayCast(Entity);

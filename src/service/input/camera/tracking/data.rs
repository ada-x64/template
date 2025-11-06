use crate::prelude::*;

/// Tracking camera. Will follow the given entity. Will spawn a CameraController on add.
/// Prefer to use [tracking_cam_bundle].
#[derive(Component, Debug, Reflect)]
#[require(CameraController::new(CameraControllerKind::Tracking), ICtxTrackingCam)]
#[component(on_add=on_add_tracking_cam)]
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
fn on_add_tracking_cam(mut world: DeferredWorld, ctx: HookContext) {
    let tracked = world
        .entity(ctx.entity)
        .get_ref::<TrackingCam>()
        .unwrap()
        .entity;
    world
        .commands()
        .entity(ctx.entity)
        .insert(Tracking(tracked));
}

#[derive(Component, Debug, Reflect)]
#[relationship(relationship_target = Tracking)]
pub struct TrackedBy {
    #[relationship]
    tracker: Entity,
}

#[derive(Component, Debug, Reflect)]
#[relationship_target(relationship = TrackedBy)]
pub struct Tracking(Entity);

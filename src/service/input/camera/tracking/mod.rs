use crate::prelude::*;

mod bundle;
mod data;
mod events;
mod systems;

pub mod prelude {
    pub use super::bundle::tracking_cam_bundle;
    pub use super::data::*;
    pub use super::systems::systems as tracking_cam_systems;
}

// The idea:
// Minimum and maximum distance spheres.
// The camera tries to maintain the maximum distance,
// but collisions with physics objects will move the camera towards the minimum distance sphere.
// The camera _cannot_ clip through the minimum distance sphere and will maintain at least that much distance.
// Zoom = changing outer sphere radius.
// No need for colliders, could just cast a single ray from the player to the desired camera position.
// If there is a collision, then (smoothly) move to the collisions location.

pub fn plugin(app: &mut App) {
    app.add_plugins(events::plugin)
        .add_input_context::<ICtxTrackingCam>();
}

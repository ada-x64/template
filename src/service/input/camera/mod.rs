use crate::prelude::*;

mod controller;
mod data;
mod fly;
mod tracking;

pub mod prelude {
    pub use super::controller::prelude::*;
    pub use super::data::*;
    pub use super::fly::prelude::*;
    pub use super::tracking::prelude::*;
    #[doc(hidden)]
    pub use bevy::camera::visibility::RenderLayers;

    use tfw::prelude::ServiceSystems;
    /// Adds in all camera systems to this screen.
    /// Currently only consists of tracking camera systems.
    pub fn camera_systems() -> ServiceSystems {
        ServiceSystems::new(tracking_cam_systems().take())
    }
}

pub fn plugin(app: &mut App) {
    app.add_plugins((tracking::plugin, fly::plugin, controller::plugin))
        .init_resource::<ActiveCamera>();
}

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
}

pub fn plugin(app: &mut App) {
    app.add_plugins((tracking::plugin, fly::plugin, controller::plugin))
        .init_resource::<CameraList>()
        .init_resource::<ActiveCamera>();
}

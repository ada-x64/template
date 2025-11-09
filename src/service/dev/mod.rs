use crate::prelude::*;

mod camera_test;
mod gizmos;
pub mod prelude {
    pub use super::camera_test::prelude::*;
    pub use super::gizmos::prelude::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(gizmos::plugin);
    app.add_plugins(camera_test::plugin);
}

mod camera_test;
use crate::prelude::*;

pub mod prelude {
    pub use super::camera_test::prelude::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(camera_test::plugin);
}

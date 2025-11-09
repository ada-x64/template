use crate::prelude::*;

mod camera_test;
mod console;
mod gizmos;
mod third_party;

pub mod prelude {
    pub use super::camera_test::prelude::*;
    // pub use super::console::prelude::*;
    pub use super::gizmos::prelude::*;
    pub use super::third_party::prelude::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(gizmos::plugin);
    app.add_plugins(camera_test::plugin);
    app.add_plugins(console::plugin);
    app.add_plugins(third_party::plugin);
}

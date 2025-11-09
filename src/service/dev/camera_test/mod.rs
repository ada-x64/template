use crate::prelude::*;

mod data;
mod systems;

pub mod prelude {
    pub use super::data::*;
    pub use super::systems::systems as camera_test_systems; // pub use super::my_submodule::prelude::*;
}

pub fn plugin(_app: &mut App) {
    // app.add_plugins(/* ... */);
}

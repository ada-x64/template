use crate::prelude::*;

mod data;
mod screen;
mod state;
mod systems;

pub mod prelude {
    pub use super::data::*;
    pub use super::screen::{CameraTestScreen, CameraTestSettings};
}

pub use systems::systems;

pub fn plugin(app: &mut App) {
    app.add_plugins((state::plugin, screen::plugin));
}

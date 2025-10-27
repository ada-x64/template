use crate::prelude::*;

mod screen;
mod state;
pub mod prelude {
    pub use super::screen::{TestScreen, TestScreenSettings};
}

pub fn plugin(app: &mut App) {
    app.add_plugins((state::plugin, screen::plugin));
}

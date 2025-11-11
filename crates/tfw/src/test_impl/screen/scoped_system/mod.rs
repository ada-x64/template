use crate::prelude::*;

mod screen;
mod state;
pub mod prelude {
    pub use super::screen::{ScopedSystemScreen, ScopedSystemSettings};
}

pub fn plugin(app: &mut App) {
    app.add_plugins((state::plugin, screen::plugin));
}

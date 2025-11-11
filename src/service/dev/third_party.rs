use crate::prelude::*;

pub mod prelude {
    pub use bevy_console::{AddConsoleCommand, ConsoleCommand};
}

pub fn plugin(app: &mut App) {
    app.add_plugins(bevy_console::ConsolePlugin);
}

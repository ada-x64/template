use crate::prelude::*;

mod commands;

pub mod prelude {}

pub fn plugin(app: &mut App) {
    app.add_plugins(commands::plugin);
}

pub(crate) mod player;
pub(crate) mod ui;
pub(crate) mod worldgen;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    // third-party
    app.add_plugins(avian3d::PhysicsPlugins::default());
    // services
    app.add_plugins((player::plugin, worldgen::plugin, ui::plugin));
}

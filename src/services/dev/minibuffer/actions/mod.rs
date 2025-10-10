// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use bevy::prelude::*;

pub(crate) mod in_world;

pub fn plugin(app: &mut App) {
    app.add_plugins(in_world::plugin);
}

// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use crate::prelude::*;

mod state;
mod systems;

pub fn plugin(app: &mut App) {
    app.add_plugins((state::plugin, systems::plugin));
}

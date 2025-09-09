// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use bevy::prelude::*;
use bevy_minibuffer::prelude::*;

use crate::services::data::GrabCursor;

pub(crate) mod perf;
pub(crate) mod world;

pub fn plugin(app: &mut App) {
    app.add_plugins((perf::plugin, world::plugin)).add_systems(
        Startup,
        |mut minibuffer: Minibuffer, mut commands: Commands| {
            minibuffer.message("Dev mode enabled.");
            minibuffer.set_visible(true);
            commands.send_event(GrabCursor::<false>);
        },
    );
}

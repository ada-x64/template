// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use bevy::prelude::*;
use bevy_minibuffer::acts::{Act, AddActs};

pub(crate) mod in_world;

fn quit_app(mut commands: Commands) {
    commands.send_event(AppExit::Success);
}

pub fn plugin(app: &mut App) {
    app.add_plugins(in_world::plugin)
        .add_acts(Act::new(quit_app).bind([KeyCode::Escape, KeyCode::Escape]));
}

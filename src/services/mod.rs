// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
pub(crate) mod data;
use data::*;

pub(crate) mod player;
pub(crate) mod ui;
pub(crate) mod worldgen;

use bevy::{prelude::*, window::CursorGrabMode};
use bevy_enhanced_input::EnhancedInputPlugin;
use bevy_rich_text3d::Text3dPlugin;
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;

fn grab_mouse(_: Trigger<GrabCursor<true>>, mut window: Single<&mut Window>) {
    window.cursor_options.visible = false;
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
}
fn release_mouse(_: Trigger<GrabCursor<false>>, mut window: Single<&mut Window>) {
    window.cursor_options.visible = true;
    window.cursor_options.grab_mode = CursorGrabMode::None;
}

pub fn plugin(app: &mut App) {
    // third-party
    app.add_plugins((
        avian3d::PhysicsPlugins::default(),
        TnuaControllerPlugin::default(),
        TnuaAvian3dPlugin::new(FixedUpdate),
        EnhancedInputPlugin,
        Text3dPlugin::default(),
    ));

    // local
    app.add_plugins((player::plugin, worldgen::plugin, ui::plugin))
        .add_event::<GrabCursor<true>>()
        .add_event::<GrabCursor<false>>()
        .add_observer(grab_mouse)
        .add_observer(release_mouse);
}

// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
pub(crate) mod actions;
pub(crate) mod fly_cam;
pub(crate) mod inspectors;

use bevy::prelude::*;
use bevy_minibuffer::prelude::*;

pub fn plugin(app: &mut App) {
    // set up
    app.add_plugins((
        bevy_inspector_egui::bevy_egui::EguiPlugin {
            enable_multipass_for_primary_context: true,
        },
        bevy_inspector_egui::DefaultInspectorConfigPlugin,
        MinibufferPlugins,
    ))
    .add_acts(BasicActs::default());

    // add functionality
    app.add_plugins((actions::plugin, inspectors::plugin, fly_cam::plugin));
}

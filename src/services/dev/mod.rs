// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
pub(crate) mod gizmos;
pub(crate) mod minibuffer;

use bevy::prelude::*;
use bevy_simple_subsecond_system::SimpleSubsecondPlugin;

pub fn plugin(app: &mut App) {
    // general utils
    app.add_plugins((
        bevy_mod_debugdump::CommandLineArgs,
        SimpleSubsecondPlugin::default(), // TODO: Switch to official hotloading in 0.17
                                          // bevy_flycam::NoCameraPlayerPlugin, // temp?
    ));

    // local functionality
    app.add_plugins((minibuffer::plugin, gizmos::plugin));
}

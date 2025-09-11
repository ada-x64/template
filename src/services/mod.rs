// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
pub(crate) mod input;
pub(crate) mod player;
pub(crate) mod ui;
pub(crate) mod worldgen;

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_rich_text3d::Text3dPlugin;
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;

pub mod prelude {
    pub use super::input::prelude::*;
    pub use super::player::prelude::*;
    pub use super::worldgen::prelude::*;
}

pub fn plugin(app: &mut App) {
    // third-party
    app.add_plugins((
        avian3d::PhysicsPlugins::default(),
        TnuaControllerPlugin::new(FixedUpdate),
        TnuaAvian3dPlugin::new(FixedUpdate),
        EnhancedInputPlugin,
        Text3dPlugin::default(),
    ));

    // local
    app.add_plugins((player::plugin, worldgen::plugin, ui::plugin, input::plugin));
}

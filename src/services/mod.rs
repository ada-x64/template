pub(crate) mod player;
pub(crate) mod ui;
pub(crate) mod worldgen;

use bevy::prelude::*;
use bevy_enhanced_input::EnhancedInputPlugin;
use bevy_rich_text3d::Text3dPlugin;
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;

pub fn plugin(app: &mut App) {
    // third-party
    app.add_plugins((
        avian3d::PhysicsPlugins::default(),
        TnuaControllerPlugin::default(),
        TnuaAvian3dPlugin::new(FixedUpdate),
        EnhancedInputPlugin,
        Text3dPlugin::default(),
    ));
    // services
    app.add_plugins((player::plugin, worldgen::plugin, ui::plugin));
}

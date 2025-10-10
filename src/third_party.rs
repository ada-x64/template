use crate::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_rich_text3d::Text3dPlugin;
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;

pub fn plugin(app: &mut App) {
    // third-party
    app.add_plugins((
        avian3d::PhysicsPlugins::default(),
        TnuaControllerPlugin::new(FixedUpdate),
        TnuaAvian3dPlugin::new(FixedUpdate),
        EnhancedInputPlugin,
        Text3dPlugin::default(),
    ));
}

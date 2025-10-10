use crate::prelude::*;

pub mod prelude {
    pub use avian3d::prelude::*;
    pub use bevy::prelude::*;
    pub use bevy_asset_loader::prelude::*;
    #[allow(unused_imports, reason = "fix ambiguous import")]
    pub use bevy_enhanced_input::prelude::Completed;
    pub use bevy_enhanced_input::prelude::*;
    pub use bevy_tnua::prelude::*;
    pub use tiny_bail::prelude::*;
}

pub fn plugin(app: &mut App) {
    // third-party
    app.add_plugins((
        avian3d::PhysicsPlugins::default(),
        TnuaControllerPlugin::new(FixedUpdate),
        bevy_tnua_avian3d::TnuaAvian3dPlugin::new(FixedUpdate),
        EnhancedInputPlugin,
        bevy_rich_text3d::Text3dPlugin::default(),
    ));
}

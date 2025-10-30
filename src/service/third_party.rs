use crate::prelude::*;

pub mod prelude {
    pub use avian3d::prelude::*;
    pub use bevy::prelude::*;
    pub use bevy_asset_loader::prelude::*;
    #[allow(unused_imports, reason = "fix ambiguous import")]
    pub use bevy_enhanced_input::prelude::Completed;
    pub use bevy_enhanced_input::prelude::*;
    #[cfg(feature = "dev")]
    pub use bevy_minibuffer::prelude::*;
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
        #[cfg(not(test))]
        bevy_rich_text3d::Text3dPlugin::default(),
    ));

    #[cfg(not(test))]
    #[cfg(feature = "dev")]
    app.add_plugins((
        bevy_minibuffer::MinibufferPlugins,
        bevy_inspector_egui::DefaultInspectorConfigPlugin,
        bevy_inspector_egui::bevy_egui::EguiPlugin {
            enable_multipass_for_primary_context: true,
        },
    ));
}

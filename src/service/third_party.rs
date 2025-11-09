use crate::{AppSettings, prelude::*};

pub mod prelude {
    pub use avian3d::prelude::*;
    pub use bevy::prelude::*;
    pub use bevy_asset_loader::prelude::*;
    pub use bevy_enhanced_input::prelude::*;
    // fix ambiguous glob exports
    pub use bevy_enhanced_input::prelude::{Cancel, Press, Release};
    pub use bevy_tnua::prelude::*;
    pub use tiny_bail::prelude::*;
}

pub fn plugin(app: &mut App) {
    let settings = app.world().resource::<AppSettings>();
    if settings.use_physics {
        app.add_plugins((
            avian3d::PhysicsPlugins::default(),
            TnuaControllerPlugin::new(FixedUpdate),
            bevy_tnua_avian3d::TnuaAvian3dPlugin::new(FixedUpdate),
        ));
    }
    app.add_plugins((
        EnhancedInputPlugin,
        #[cfg(not(test))]
        bevy_rich_text3d::Text3dPlugin::default(),
    ));
}

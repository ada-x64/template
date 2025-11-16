#![doc = include_str!("./doc.md")]

use bevy::app::HierarchyPropagatePlugin;

use crate::{TfwSettings, prelude::*};

mod data;
mod scope;
mod trait_impl;

pub mod prelude {
    pub use super::data::*;
    pub use super::scope::*;
    pub use super::trait_impl::*;
    pub use bevy_asset_loader::prelude::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins((
        HierarchyPropagatePlugin::<Persistent>::new(PostUpdate),
        HierarchyPropagatePlugin::<ScreenScoped>::new(PostUpdate),
    ));
    // This occurs _after_ registration
    app.add_systems(
        Startup,
        |registry: Res<ScreenRegistry>, settings: Res<TfwSettings>, mut commands: Commands| {
            let name = &settings.initial_screen;
            let system_id = registry
                .get(name)
                .unwrap_or_else(|| panic!("Invalid initial screen {name:?}"));
            commands.run_system(*system_id);
        },
    );
}

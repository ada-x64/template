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
    let settings = app.world().resource::<TfwSettings>();
    app.insert_state::<CurrentScreen>(settings.initial_screen.into());
    app.insert_state::<CurrentScreenStatus>(ScreenStatus::Loading.into());
    app.init_resource::<NextScreen>();

    app.add_plugins((
        HierarchyPropagatePlugin::<Persistent>::new(PostUpdate),
        HierarchyPropagatePlugin::<ScreenScoped>::new(PostUpdate),
    ));
}

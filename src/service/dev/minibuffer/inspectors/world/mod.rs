use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::prelude::*;

mod data;
mod systems;

pub mod prelude {
    pub use super::data::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(systems::plugin);
    app.add_plugins(
        WorldInspectorPlugin::default()
            .run_if(in_state(WorldInspectorState::Visible).and(in_state(PromptState::Visible))),
    );
    app.init_state::<WorldInspectorState>();
}

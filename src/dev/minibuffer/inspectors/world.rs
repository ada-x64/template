// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_minibuffer::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
enum WorldInspectorState {
    #[default]
    Invisible,
    Visible,
}

fn inspect_world(
    state: Res<State<WorldInspectorState>>,
    mut next_state: ResMut<NextState<WorldInspectorState>>,
    mut minibuffer: Minibuffer,
) {
    use WorldInspectorState::*;
    let state = match state.get() {
        Invisible => Visible,
        Visible => Invisible,
    };
    next_state.set(state);
    minibuffer.clear();
}

pub fn plugin(app: &mut App) {
    app.add_plugins(
        WorldInspectorPlugin::default()
            .run_if(in_state(WorldInspectorState::Visible).and(in_state(PromptState::Visible))),
    )
    .add_acts((Act::new(inspect_world).bind([KeyCode::F1]),))
    .init_state::<WorldInspectorState>();
}

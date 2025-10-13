pub use crate::prelude::*;

fn inspect_world(
    state: Res<State<WorldInspectorState>>,
    mut next_state: ResMut<NextState<WorldInspectorState>>,
    mut minibuffer: Minibuffer,
) {
    use super::data::WorldInspectorState::*;
    let state = match state.get() {
        Invisible => Visible,
        Visible => Invisible,
    };
    next_state.set(state);
    minibuffer.clear();
}

pub fn plugin(app: &mut App) {
    // minibuffer should probably use observers for these.
    app.add_acts((Act::new(inspect_world).bind([KeyCode::F1]),));
}

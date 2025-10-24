use crate::prelude::*;

pub fn plugin(app: &mut App) {
    let state = CurrentScreen::new::<WorldScreen>();
    app.add_loading_state(
        LoadingState::new(state.loading())
            .continue_to_state(state.ready())
            .load_collection::<PlayerAssets>(),
    );
}

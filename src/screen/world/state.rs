use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(ScreenState::<WorldScreen>::Loading)
            .continue_to_state(ScreenState::<WorldScreen>::Ready)
            .load_collection::<PlayerAssets>(),
    );
}

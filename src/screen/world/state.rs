use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(ScreenLoadingState::<WorldScreen>::Loading)
            .continue_to_state(ScreenLoadingState::<WorldScreen>::Ready)
            .load_collection::<PlayerAssets>(),
    );
}

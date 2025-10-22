use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screens::InWorld(ScreenStatus::Loading))
            .continue_to_state(Screens::InWorld(ScreenStatus::Ready))
            .load_collection::<PlayerAssets>(),
    );
}

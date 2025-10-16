use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<WorldScreenStates>();
    app.add_loading_state(
        LoadingState::new(WorldScreenStates::Loading)
            .continue_to_state(WorldScreenStates::Ready)
            .load_collection::<PlayerAssets>(),
    );
}

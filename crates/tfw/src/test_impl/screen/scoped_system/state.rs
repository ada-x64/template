use crate::prelude::*;

#[derive(AssetCollection, Resource)]
struct ScopedSystemScreenAssetCollection {}

pub fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(ScreenLoadingState::<ScopedSystemScreen>::Loading)
            .continue_to_state(ScreenLoadingState::<ScopedSystemScreen>::Ready)
            .load_collection::<ScopedSystemScreenAssetCollection>(),
    );
}

use crate::prelude::*;

/// TODO: Use Skein or granite while awaiting bsn?
#[derive(AssetCollection, Resource)]
struct CameraTestScreenAssetCollection {}

pub fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(ScreenLoadingState::<CameraTestScreen>::Loading)
            .continue_to_state(ScreenLoadingState::<CameraTestScreen>::Ready)
            .load_collection::<CameraTestScreenAssetCollection>(),
    );
}

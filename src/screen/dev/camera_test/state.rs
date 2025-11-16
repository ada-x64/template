use crate::prelude::*;

/// TODO: Use Skein or granite while awaiting bsn?
#[derive(AssetCollection, Resource)]
struct CameraTestScreenAssetCollection {}

pub fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(ScreenState::<CameraTestScreen>::Loading)
            .continue_to_state(ScreenState::<CameraTestScreen>::Ready)
            .load_collection::<CameraTestScreenAssetCollection>(),
    );
}

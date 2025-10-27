use crate::prelude::*;

/// This plugin controls how the state transitions from Loading to Ready.
/// It does not have to use [bevy_asset_loader], but it's helpful to do so.
pub fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(CurrentScreen::new::<TestScreen>())
            .continue_to_state(CurrentScreen::new::<TestScreen>().loading())
            .load_collection::<EmptyAssetCollection>(),
    );
}

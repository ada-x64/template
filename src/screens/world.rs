// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{prelude::*, services::player::assets::PlayerAssets};

pub fn plugin(app: &mut App) {
    app.init_state::<WorldScreenStates>()
        .add_loading_state(
            LoadingState::new(WorldScreenStates::Loading)
                .continue_to_state(WorldScreenStates::Ready)
                .load_collection::<PlayerAssets>(),
        )
        .add_systems(
            OnEnter(WorldScreenStates::Ready),
            (|mut commands: Commands| {
                commands.trigger(SpawnPlayerRoot);
                commands.trigger(SpawnWorldgenRoot);
            })
            .run_if(in_state(ScreenStates::InWorld)),
        );
}

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    screens::ScreenStates,
    services::{
        player::{assets::PlayerAssets, data::SpawnPlayerRoot},
        worldgen::data::SpawnWorldgenRoot,
    },
};

#[derive(States, PartialEq, Eq, Hash, Debug, Clone, Copy, Reflect, Default)]
pub enum WorldScreenStates {
    #[default]
    Loading,
    Ready,
}

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

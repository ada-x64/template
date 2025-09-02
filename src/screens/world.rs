use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    screens::ScreenStates,
    services::{
        player::{assets::PlayerAssets, spawn_player_root},
        worldgen::spawn_worldgen_root,
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
            (spawn_player_root, spawn_worldgen_root).run_if(in_state(ScreenStates::InWorld)),
        );
}

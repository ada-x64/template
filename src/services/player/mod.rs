pub(crate) mod assets;
pub(crate) mod camera;
pub(crate) mod controller;
pub(crate) mod data;

use bevy::prelude::*;
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;

use crate::{
    screens::ScreenStates,
    services::player::{
        camera::player_cam,
        controller::{player_controller, spawn_player_mesh},
        data::SpawnPlayerRoot,
    },
};

#[derive(Component)]
pub struct PlayerRoot;
pub fn spawn_player_root(_: Trigger<SpawnPlayerRoot>, mut commands: Commands) {
    commands.spawn((
        PlayerRoot,
        Transform::from_xyz(0., 0., 0.), // TODO: Should match terrain.
        Visibility::Visible,
        Name::new("PlayerRoot"),
        StateScoped(ScreenStates::InWorld),
    ));
    commands.spawn(player_cam());
    commands.spawn(player_controller());
    commands.run_system_cached(spawn_player_mesh);
}

pub fn plugin(app: &mut App) {
    app.add_plugins((
        TnuaControllerPlugin::default(),
        TnuaAvian3dPlugin::new(FixedUpdate),
    ))
    .add_observer(spawn_player_root);
}

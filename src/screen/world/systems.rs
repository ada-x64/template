use crate::prelude::*;

#[derive(AssetCollection, Resource, Default, Debug)]
pub struct WorldAssets {
    pub player_assets: PlayerAssets,
}

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Reflect)]
pub struct WorldScreen;
impl Screen for WorldScreen {
    type SETTINGS = NoSettings;
    type ASSETS = WorldAssets;
    const STRATEGY: LoadingStrategy = LoadingStrategy::Blocking;
}

fn init(mut commands: Commands) {
    debug!("in world: init");
    commands.trigger(SpawnPlayerRoot);
    commands.trigger(SpawnWorldgenRoot);
}

pub fn plugin(app: &mut App) {
    ScreenScopeBuilder::<WorldScreen>::new(app)
        .on_ready(init)
        .add_systems(player_systems().take())
        .add_systems(tracking_cam_systems().take())
        .build();
}

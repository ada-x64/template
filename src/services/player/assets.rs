use bevy::{asset::RenderAssetUsages, gltf::GltfLoaderSettings, prelude::*};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(path = "models/basil.glb#Scene0")]
    pub model: Handle<Scene>,
}

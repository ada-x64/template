use bevy::reflect::enum_hash;
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::prelude::*;

#[derive(Default, PartialEq, Eq, Hash, Debug, Clone, Copy, Reflect)]
pub enum Screens {
    #[default]
    Empty,
    NamedEntity,
}
impl From<Screens> for ScreenType {
    fn from(value: Screens) -> Self {
        enum_hash(&value).unwrap().into()
    }
}

#[derive(AssetCollection, Resource)]
pub struct EmptyAssetCollection {}

use bevy_asset_loader::asset_collection::AssetCollection;

use crate::prelude::*;

#[derive(Default, PartialEq, Eq, Hash, Debug, Clone, Copy, Reflect)]
pub enum Screens {
    #[default]
    Empty,
    NamedEntity,
}
impl Screens {
    pub const fn as_screen_type(self) -> ScreenType {
        let val = match self {
            Screens::Empty => "empty",
            Screens::NamedEntity => "named_entity",
        };
        ScreenType(val)
    }
}
impl From<Screens> for ScreenType {
    fn from(value: Screens) -> Self {
        value.as_screen_type()
    }
}

#[derive(AssetCollection, Resource)]
pub struct EmptyAssetCollection {}

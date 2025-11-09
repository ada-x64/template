use bevy::reflect::enum_hash;

use crate::prelude::*;

/// Enumeration of all screens within the app.
/// Screens represent "sub-simulations" which scope
/// systems, events, and entites. See the docs
/// for more info.
#[derive(Default, PartialEq, Eq, Hash, Debug, Clone, Copy, Reflect)]
pub enum Screens {
    #[default]
    Splash,
    MainMenu,
    World,
    #[cfg(feature = "dev")]
    CameraTest,
}
impl From<Screens> for ScreenType {
    fn from(value: Screens) -> Self {
        enum_hash(&value).unwrap().into()
    }
}

#[derive(AssetCollection, Resource)]
pub struct EmptyAssetCollection {}

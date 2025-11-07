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
    CameraTest,
}
impl Screens {
    pub const fn as_screen_type(self) -> ScreenType {
        let val = match self {
            Screens::CameraTest => "camera_test",
            Screens::Splash => "splash",
            Screens::MainMenu => "main_menu",
            Screens::World => "world",
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

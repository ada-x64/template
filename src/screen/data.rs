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
    InWorld,
    CameraTest,
    Test,
}

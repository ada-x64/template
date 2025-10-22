use crate::prelude::*;

/// Enumeration of all screens within the app.
/// Screens represent "sub-simulations" which scope
/// systems, events, and entites. See the docs
/// for more info.
#[derive(States, PartialEq, Eq, Hash, Debug, Clone, Copy, Reflect)]
pub enum Screens {
    Splash(ScreenStatus),
    MainMenu(ScreenStatus),
    InWorld(ScreenStatus),
    CameraTest(ScreenStatus),
}
impl Default for Screens {
    fn default() -> Self {
        Self::Splash(ScreenStatus::default())
    }
}

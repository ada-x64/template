use crate::prelude::*;

/// Loading is handled within the individual screens.
#[derive(States, PartialEq, Eq, Hash, Debug, Clone, Copy, Reflect, Default)]
pub enum ScreenStates {
    // #[cfg_attr(not(feature="dev"), default)]
    Splash,
    MainMenu,
    #[default] // #[cfg_attr(feature="dev", default)]
    InWorld,
}

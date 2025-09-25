use crate::prelude::*;

#[derive(States, PartialEq, Eq, Hash, Debug, Clone, Copy, Reflect, Default)]
pub enum WorldScreenStates {
    #[default]
    Loading,
    Ready,
}

/// Loading is handled within the individual screens.
#[derive(States, PartialEq, Eq, Hash, Debug, Clone, Copy, Reflect)]
pub enum ScreenStates {
    Splash,
    MainMenu,
    InWorld,
}

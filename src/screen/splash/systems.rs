use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Reflect)]
pub struct SplashScreen;
impl Screen for SplashScreen {
    type SETTINGS = NoSettings;
    type ASSETS = NoAssets;
    const STRATEGY: LoadingStrategy = LoadingStrategy::Nonblocking;
}

pub fn plugin(app: &mut App) {
    ScreenScopeBuilder::<SplashScreen>::new(app).build();
}

use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SplashScreen;

impl Screen for SplashScreen {
    type SETTINGS = EmptySettings;
    type ASSETS = NoAssets;

    fn options() -> ScreenOptions {
        ScreenOptions {
            name: Screens::Splash.into(),
            strategy: LoadingStrategy::Nonblocking,
        }
    }
}

pub fn plugin(app: &mut App) {
    ScreenScopeBuilder::<SplashScreen>::new(app).build();
}

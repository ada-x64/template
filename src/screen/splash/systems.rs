use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SplashScreen;

impl Screen for SplashScreen {
    const NAME: Screens = Screens::Splash;
    type SETTINGS = EmptySettings;
    fn init<'w>(_world: DeferredWorld<'w>, _ctx: HookContext) {
        debug!("init (splash)")
    }
}

pub fn plugin(app: &mut App) {
    ScreenScopeBuilder::<SplashScreen>::fixed().build(app);
}

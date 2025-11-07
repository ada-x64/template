use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct EmptyScreen;
impl Screen for EmptyScreen {
    const NAME: ScreenType = Screens::Empty.as_screen_type();
    type SETTINGS = EmptySettings;
    fn init<'w>(_world: DeferredWorld<'w>, _ctx: HookContext) {
        debug!("in init (empty)");
    }
}

pub fn plugin(app: &mut App) {
    debug!("in test plugin");
    ScreenScopeBuilder::<EmptyScreen>::fixed().build(app);
}

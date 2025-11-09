use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct EmptyScreen;
impl Screen for EmptyScreen {
    type SETTINGS = EmptySettings;
    fn name() -> ScreenType {
        Screens::Empty.into()
    }
    fn init<'w>(_world: DeferredWorld<'w>, _ctx: HookContext) {
        debug!("in init (empty)");
    }
}

pub fn plugin(app: &mut App) {
    debug!("in test plugin");
    ScreenScopeBuilder::<EmptyScreen>::fixed().build(app);
}

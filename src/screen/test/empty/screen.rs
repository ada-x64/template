use crate::prelude::*;
use bevy::ecs::{component::HookContext, world::DeferredWorld};

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct EmptyScreen;
impl Screen for EmptyScreen {
    const NAME: Screens = Screens::Empty;
    type SETTINGS = EmptySettings;
    fn init<'w>(_world: DeferredWorld<'w>, _ctx: HookContext) {
        debug!("in init (empty)");
    }
}

pub fn plugin(app: &mut App) {
    debug!("in test plugin");
    ScreenScopeBuilder::<EmptyScreen>::fixed().build(app);
}

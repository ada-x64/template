use crate::prelude::*;
use bevy::ecs::{component::HookContext, world::DeferredWorld};

/// The main [Screen] implementation.
#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct TestScreen;
impl Screen for TestScreen {
    const NAME: Screens = Screens::Test;

    /// Use this optional function to initialize your screen, e.g. by calling commands
    /// or scoping observers.
    fn init<'w>(world: &mut DeferredWorld<'w>, _ctx: &HookContext) {
        info!("in init");
        world.commands().spawn(Name::new("Hello"));
    }

    /// Use this optional function to handle any unloading logic, e.g.
    /// despawning entities, serializing state, etc.
    /// (Note: In most cases, you should scope entities by using the
    /// [ScreenScoped] component)
    fn unload(_world: &mut World) {}
}

pub fn plugin(app: &mut App) {
    info!("in test plugin");
    TestScreen::builder_fixed().build(app);
}

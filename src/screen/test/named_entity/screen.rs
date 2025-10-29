use crate::prelude::*;
use bevy::ecs::{component::HookContext, world::DeferredWorld};

#[derive(PartialEq, Eq, Clone, Debug, Hash, Reflect, Default, Resource)]
pub struct NamedEntityScreenSettings {
    pub entity_name: String,
}

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct NamedEntityScreen;
impl Screen for NamedEntityScreen {
    const NAME: Screens = Screens::NamedEntity;
    type SETTINGS = NamedEntityScreenSettings;
    fn init<'w>(mut world: DeferredWorld<'w>, _ctx: HookContext) {
        debug!("in init (Test)");
        let settings = world
            .get_resource::<Self::SETTINGS>()
            .expect("Settings should be initialized in plugin.")
            .clone();
        world
            .commands()
            .spawn(Name::new(settings.entity_name.clone()));
    }
}

pub fn plugin(app: &mut App) {
    debug!("in test plugin");
    ScreenScopeBuilder::<NamedEntityScreen>::fixed().build(app);
}

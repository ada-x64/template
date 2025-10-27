use bevy::ecs::{component::HookContext, world::DeferredWorld};

use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct WorldScreen;
impl Screen for WorldScreen {
    const NAME: Screens = Screens::World;
    type SETTINGS = EmptySettings;

    fn init<'w>(world: &mut DeferredWorld<'w>, _ctx: &HookContext) {
        info!("in world: init");
        let mut commands = world.commands();
        commands.trigger(SpawnPlayerRoot);
        commands.trigger(SpawnWorldgenRoot);
    }
}

pub fn plugin(app: &mut App) {
    WorldScreen::builder_fixed()
        .add_systems(player_systems().take())
        .add_systems(tracking_cam_systems().take())
        .build(app);
}

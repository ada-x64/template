use bevy::ecs::{component::HookContext, schedule::ScheduleLabel, world::DeferredWorld};

use crate::prelude::*;

#[derive(ScheduleLabel, SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorldSchedule {
    FixedUpdate,
}

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct WorldScreen;
impl Screen for WorldScreen {
    fn init<'w>(world: &mut DeferredWorld<'w>, _ctx: &HookContext) {
        info!("in world: init");
        let mut commands = world.commands();
        commands.trigger(SpawnPlayerRoot);
        commands.trigger(SpawnWorldgenRoot);
    }
}

pub fn plugin(app: &mut App) {
    ScreenScope::<WorldScreen>::default()
        .builder(
            WorldSchedule::FixedUpdate,
            Screens::InWorld(ScreenStatus::Ready),
        )
        .add_systems(player_systems().take())
        .add_systems(tracking_cam_systems().take())
        .build_fixed(app);
}

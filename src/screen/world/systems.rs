use bevy::ecs::{component::HookContext, schedule::ScheduleLabel, world::DeferredWorld};

use crate::prelude::*;

#[derive(ScheduleLabel, SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorldSchedule {
    FixedUpdate,
}

#[derive(Component, Debug, Clone, Copy, Default)]
#[component(on_add = on_add)]
pub struct WorldScreen;

fn on_add<'w>(mut world: DeferredWorld<'w>, _ctx: HookContext) {
    let mut commands = world.commands();
    commands.trigger(SpawnPlayerRoot);
    commands.trigger(SpawnWorldgenRoot);
    // scoped observers
    // commands.entity(ctx.entity).observe(player_observers().take());
}

pub fn plugin(app: &mut App) {
    ScreenScope::<WorldScreen>::default()
        .builder(WorldSchedule::FixedUpdate, WorldScreenStates::Ready)
        .add_systems((player_systems().take(), tracking_cam_systems().take()))
        .build_fixed(app);
}

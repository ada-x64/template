use crate::prelude::*;
use bevy_inspector_egui::bevy_inspector::{
    guess_entity_name,
    hierarchy::{Hierarchy, SelectedEntities},
};

pub fn log_hierarchy(world: &mut World) {
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    let type_registry = type_registry.read();
    let h = Hierarchy {
        world,
        type_registry: &type_registry,
        selected: &mut SelectedEntities::default(),
        context_menu: None,
        shortcircuit_entity: None,
        extra_state: &mut (),
    };

    let mut root_query = h
        .world
        .query_filtered::<Entity, (Without<ChildOf>, Without<Observer>)>();

    let mut entities: Vec<_> = root_query.iter(h.world).collect();
    entities.sort();

    let mut output = String::new();
    for &entity in &entities {
        let entity_name = guess_entity_name(world, entity);
        output = format!("{output}\n{entity_name}");
    }
    info!("{output}")
}

pub fn switch_screen(app: &mut App, screen: Screens) {
    app.world_mut().trigger(SwitchToScreen(screen));
    info!("SwitchToScreen({screen:?}) (about to update)");
    app.update();
    log_hierarchy(app.world_mut());
}

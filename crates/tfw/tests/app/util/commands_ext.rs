use bevy_inspector_egui::bevy_inspector::{
    guess_entity_name,
    hierarchy::{Hierarchy, SelectedEntities},
};

use crate::prelude::*;

pub trait CommandsExt {
    fn log_hierarchy(&mut self);
}
impl<'w, 's> CommandsExt for Commands<'w, 's> {
    fn log_hierarchy(&mut self) {
        self.queue(|world: &mut World| {
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

            let mut root_query = h.world.query_filtered::<Entity, Without<ChildOf>>();
            let entities: Vec<_> = root_query.iter(h.world).collect();
            let mut output = String::new();
            log_hierarchy_inner(world, &mut output, entities, 0);
            info!("{output}")
        });
    }
}

fn log_hierarchy_inner(world: &mut World, output: &mut String, entities: Vec<Entity>, depth: u32) {
    for &entity in &entities {
        let entity_name = guess_entity_name(world, entity);
        let mut tags = vec![];
        if world.entity(entity).get::<Persistent>().is_some() {
            tags.push("Persistent");
        }
        if world.entity(entity).get::<Observer>().is_some() {
            tags.push("Observer");
        }
        let indent = (0..depth).map(|_| "-").collect::<Vec<_>>().join("");
        #[allow(clippy::obfuscated_if_else)]
        let tags = (!tags.is_empty())
            .then(|| format!("<{}>", tags.join(", ")))
            .unwrap_or_default();

        *output = format!("{output}\n{indent}> {entity_name} {tags}");

        if let Some(children) = world.entity(entity).get::<Children>() {
            let children = children.iter().collect::<Vec<Entity>>();
            log_hierarchy_inner(world, output, children, depth + 1);
        }
    }
}

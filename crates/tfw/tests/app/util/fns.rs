use crate::prelude::*;
use bevy::ecs::query::QueryFilter;
use bevy_inspector_egui::bevy_inspector::{
    guess_entity_name,
    hierarchy::{Hierarchy, SelectedEntities},
};

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

    let mut root_query = h.world.query_filtered::<Entity, Without<ChildOf>>();
    let entities: Vec<_> = root_query.iter(h.world).collect();
    let mut output = String::new();
    log_hierarchy_inner(world, &mut output, entities, 0);
    info!("{output}")
}

#[derive(Default)]
pub struct SwitchScreenOpts {
    pub log_hierarchy: bool,
    pub update: bool,
}
impl SwitchScreenOpts {
    pub fn with_log_heirarchy(mut self) -> Self {
        self.log_hierarchy = true;
        self
    }
    pub fn with_update(mut self) -> Self {
        self.update = true;
        self
    }
    pub fn full() -> Self {
        Self {
            log_hierarchy: true,
            update: true,
        }
    }
}

/// Searches for an entity with the given [Name] component.
/// This _will not_ show entities marked with [Internal], including Observers.
pub fn find_entity(app: &mut App, name: impl ToString) -> bool {
    let mut q = app.world_mut().query::<(Entity, &Name)>();
    q.iter(app.world()).for_each(|(entity, ename)| {
        debug!("found entity '{ename}' ({entity:?})");
    });
    q.iter(app.world())
        .any(|(_, ename)| (**ename).eq(&name.to_string()))
}

/// Searches for an entity with the given [Name] component
pub fn find_entity_filtered<F: QueryFilter>(app: &mut App, name: impl ToString) -> bool {
    let mut q = app.world_mut().query_filtered::<&Name, F>();
    q.iter(app.world())
        .any(|ename| (**ename).eq(&name.to_string()))
}
/// Searches for an entity with the given [Name] and component C.
pub fn _find_entity_with<C: Component + PartialEq>(
    app: &mut App,
    name: impl ToString,
    value: C,
) -> bool {
    let mut q = app.world_mut().query::<(&Name, &C)>();
    q.iter(app.world())
        .any(|(ename, c)| (**ename).eq(&name.to_string()) && *c == value)
}

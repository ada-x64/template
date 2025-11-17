use crate::prelude::*;
use bevy::ecs::query::QueryFilter;

pub trait AppExt {
    fn find_entity(&mut self, name: impl ToString) -> bool;
    fn find_entity_filtered<F: QueryFilter>(&mut self, name: impl ToString) -> bool;
    fn find_entity_with<C: Component + PartialEq>(&mut self, name: impl ToString, value: C)
    -> bool;
    fn add_step<M>(&mut self, step: u32, system: impl IntoSystem<(), (), M>) -> &mut Self;
}
impl AppExt for App {
    /// Searches for an entity with the given [Name] component.
    /// This _will not_ show entities marked with [Internal], including Observers.
    fn find_entity(&mut self, name: impl ToString) -> bool {
        let mut q = self.world_mut().query::<(Entity, &Name)>();
        q.iter(self.world()).for_each(|(entity, ename)| {
            debug!("found entity '{ename}' ({entity:?})");
        });
        q.iter(self.world())
            .any(|(_, ename)| (**ename).eq(&name.to_string()))
    }

    /// Searches for an entity with the given [Name] component
    fn find_entity_filtered<F: QueryFilter>(&mut self, name: impl ToString) -> bool {
        let mut q = self.world_mut().query_filtered::<&Name, F>();
        q.iter(self.world())
            .any(|ename| (**ename).eq(&name.to_string()))
    }
    /// Searches for an entity with the given [Name] and component C.
    fn find_entity_with<C: Component + PartialEq>(
        &mut self,
        name: impl ToString,
        value: C,
    ) -> bool {
        let mut q = self.world_mut().query::<(&Name, &C)>();
        q.iter(self.world())
            .any(|(ename, c)| (**ename).eq(&name.to_string()) && *c == value)
    }

    /// Registers a system which runs in PostUpdate (after all screen events have occured).
    /// Will only run if the state is set to the specified value.
    fn add_step<M>(&mut self, step: u32, system: impl IntoSystem<(), (), M>) -> &mut Self {
        self.add_systems(PostUpdate, system.run_if(in_state(Step(step))))
    }
}

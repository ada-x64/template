use crate::prelude::*;
use std::marker::PhantomData;

/// Extends the [App] with propogation registration functionality.
pub trait PropogateExt {
    fn register_propagatable_type<T: Propogatable>(&mut self) -> &mut Self;
}
impl PropogateExt for App {
    /// Registers a type as [Propogatable]. Use [`Propogate<T>`] to duplicate a
    /// component down the parent-child hierarchy.
    fn register_propagatable_type<T: Propogatable>(&mut self) -> &mut Self {
        self.add_systems(PostUpdate, read_propogate_message::<T>)
            .add_message::<PropogateMessage<T>>()
    }
}

/// Marks a component as able to propogate. Use [`Propogate<T>`] to duplicate a
/// component down the parent-child hierarchy.
///
/// __NOTE: Propogatable types must be registered using [App::register_propagatable_type].__
pub trait Propogatable: Component + Clone + Default + std::fmt::Debug {}
impl<T> Propogatable for T where T: Component + Clone + Default + std::fmt::Debug {}

/// Inserts the inner component and recursively clones the passed component to
/// all of its children. Use the [BlockPropogation] component to end
/// propogation.
///
/// __NOTE: Propogatable types must be registered using [App::register_propagatable_type].__
#[derive(Component, Clone, Debug, Default)]
#[component(on_insert = send_propogate_event::<T>)]
#[component(immutable)]
pub struct Propogate<T: Propogatable>(pub T);

fn send_propogate_event<'w, T: Propogatable>(mut world: DeferredWorld<'w>, ctx: HookContext) {
    debug!("Inserting propogate and sending propogate event");
    if let Some(mut q) = world.try_query::<&Propogate<T>>()
        && let Ok(propogate) = q.get(&world, ctx.entity).cloned()
    {
        world.commands().entity(ctx.entity).insert(propogate.0);
    } else {
        error!("Could not insert propogate on entity {}", ctx.entity);
    }
    world.write_message(PropogateMessage {
        target: ctx.entity,
        _marker: PhantomData::<T>,
    });
}

/// Ensures that [propogation](Propogate) of components of type T stops at this child.
/// Note that this does not stop _all_ propogation, but _only_ propogation of the specific type.
#[derive(Component, Clone, Debug, Default)]
#[component(immutable)]
pub struct BlockPropogation<T: Propogatable>(PhantomData<T>);

#[derive(Event, Clone, Debug, Deref, Message)]
struct PropogateMessage<T> {
    #[deref]
    target: Entity,
    _marker: PhantomData<T>,
}

fn read_propogate_message<T: Propogatable>(
    mut reader: MessageReader<PropogateMessage<T>>,
    mut commands: Commands,
    query: Query<(&Propogate<T>, &Children)>,
) {
    debug!("Reading propogate events (postupdate)");
    for event in reader.read() {
        if let Ok((propogate, children)) = query.get(event.target) {
            for child in children {
                commands.propogate(propogate.clone(), *child);
            }
        }
    }
}

trait CmdPropogateExt<T: Propogatable> {
    fn propogate(&mut self, value: Propogate<T>, target: Entity);
}
impl<'w, 's, T: Propogatable> CmdPropogateExt<T> for Commands<'w, 's> {
    fn propogate(&mut self, value: Propogate<T>, target: Entity) {
        self.queue(move |world: &mut World| {
            if world
                .query::<&BlockPropogation<T>>()
                .get(world, target)
                .is_ok()
            {
                debug!("Propogation blocked on child {target:?}");
                return;
            }
            debug!("Inserting propgate {value:?} on child {target:?}");
            world.commands().entity(target).insert(value.clone());
            let children = world.query::<&Children>().get(world, target);
            if let Ok(children) = children {
                for child in children.iter().collect::<Vec<Entity>>() {
                    world.commands().propogate(value.clone(), child);
                }
            }
        });
    }
}

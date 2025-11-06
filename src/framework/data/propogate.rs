use crate::prelude::*;
use std::marker::PhantomData;

/// Extends the [App] with propogation registration functionality.
pub trait PropagateExt {
    fn register_propagatable_type<T: Propagatable>(&mut self) -> &mut Self;
}
impl PropagateExt for App {
    /// Registers a type as [Propagatable]. Use [`Propagate<T>`] to duplicate a
    /// component down the parent-child hierarchy.
    fn register_propagatable_type<T: Propagatable>(&mut self) -> &mut Self {
        self.add_systems(PostUpdate, read_propogate_message::<T>)
            .add_message::<PropagateMessage<T>>()
    }
}

/// Marks a component as able to propogate. Use [`Propagate<T>`] to duplicate a
/// component down the parent-child hierarchy. Be aware that this goes _down_
/// the hierarchy, unlike [EntityEvent] propagation!
///
/// __NOTE: Propagatable types must be registered using [App::register_propagatable_type].__
pub trait Propagatable: Component + Clone + Default + std::fmt::Debug {}
impl<T> Propagatable for T where T: Component + Clone + Default + std::fmt::Debug {}

/// Inserts the inner component and recursively clones the passed component to
/// all of its children. Use the [BlockPropagation] component to end
/// propogation.
///
/// __NOTE: Propagatable types must be registered using [App::register_propagatable_type].__
#[derive(Component, Clone, Debug, Default)]
#[component(on_insert = send_propogate_event::<T>)]
#[component(immutable)]
pub struct Propagate<T: Propagatable>(pub T);

fn send_propogate_event<'w, T: Propagatable>(mut world: DeferredWorld<'w>, ctx: HookContext) {
    debug!("Inserting propogate and sending propogate event");
    if let Some(mut q) = world.try_query::<&Propagate<T>>()
        && let Ok(propogate) = q.get(&world, ctx.entity).cloned()
    {
        world.commands().entity(ctx.entity).insert(propogate.0);
    } else {
        error!("Could not insert propogate on entity {}", ctx.entity);
    }
    world.write_message(PropagateMessage {
        target: ctx.entity,
        _marker: PhantomData::<T>,
    });
}

/// Ensures that [propogation](Propagate) of components of type T stops at this child.
/// Note that this does not stop _all_ propogation, but _only_ propogation of the specific type.
#[derive(Component, Clone, Debug, Default)]
#[component(immutable)]
pub struct BlockPropagation<T: Propagatable>(PhantomData<T>);

#[derive(Event, Clone, Debug, Deref, Message)]
struct PropagateMessage<T> {
    #[deref]
    target: Entity,
    _marker: PhantomData<T>,
}

fn read_propogate_message<T: Propagatable>(
    mut reader: MessageReader<PropagateMessage<T>>,
    mut commands: Commands,
    query: Query<(&Propagate<T>, &Children)>,
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

trait CmdPropagateExt<T: Propagatable> {
    fn propogate(&mut self, value: Propagate<T>, target: Entity);
}
impl<'w, 's, T: Propagatable> CmdPropagateExt<T> for Commands<'w, 's> {
    fn propogate(&mut self, value: Propagate<T>, target: Entity) {
        self.queue(move |world: &mut World| {
            if world
                .query::<&BlockPropagation<T>>()
                .get(world, target)
                .is_ok()
            {
                debug!("Propagation blocked on child {target:?}");
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

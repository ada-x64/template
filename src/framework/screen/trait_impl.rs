use bevy::ecs::{component::HookContext, system::FunctionSystem, world::DeferredWorld};
#[allow(unused_imports, reason = "used for docs")]
use bevy::{
    app::{FixedMain, FixedMainScheduleOrder, MainScheduleOrder},
    ecs::{schedule::ScheduleLabel, system::ScheduleSystem},
};

pub use crate::prelude::*;

/// Implementation trait for Screen components.
/// ## Lifecycle
/// Screens have two lifecycle functions.
///
/// The first is [Screen::init]
/// which is called when the screen component's `on_add` hook is fired.
/// This is meant to register systems and scoped observers.
///
/// The second lifecycle function is [Screen::unload]. This function
/// is called before the screen unloads and is designed to run
/// any cleanup logic before transitioning.
pub trait Screen:
    Sized + Default + std::fmt::Debug + Clone + Copy + Eq + std::hash::Hash + Send + Sync + 'static
{
    /// The associated screen name.
    const NAME: Screens;

    /// The associated settings type. Set as () for no settings.
    type SETTINGS: Resource + FromWorld;

    /// Used as the component wrapper's [on_add
    /// hook.](https://docs.rs/bevy/latest/bevy/prelude/trait.Component.html#adding-components-hooks)
    /// Use this to scope systems and observers. In order to get settings, use
    /// `world.get_resource::<Self::SETTINGS>();`
    fn init<'w>(_world: DeferredWorld<'w>, _ctx: HookContext) {}

    /// Called when the screen is about to unload.
    /// Use this to perform any necessary cleanup before the screen transitions.
    fn unload() -> Option<impl System<In = (), Out = ()>> {
        None::<FunctionSystem<fn(), fn()>>
    }
}

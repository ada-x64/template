use bevy::ecs::{
    component::{ComponentHook, HookContext, Immutable, StorageType},
    world::DeferredWorld,
};
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

    /// Called during the on_add hook.
    /// Use this to scope systems and observers.
    /// In order to get settings, use world.get_resource::<Self::SETTINGS>();
    fn init<'w>(_world: &mut DeferredWorld<'w>, _ctx: &HookContext) {}
    /// Called when the screen is about to unload.
    /// Use this to perform any necessary cleanup before the screen transitions.
    fn unload(_world: &mut World) {}

    /// INTERNAL
    fn on_unload(_trigger: Trigger<SwitchToScreen>, mut commands: Commands) {
        info!("on_unload");
        commands.queue(Self::unload);
        commands.queue(|world: &mut World| world.trigger(FinishUnload));
    }

    /// Creates a new [ScreenScopeBuilder] which will run in [Main] after
    /// [Update]. To run on a fixed schedule, call [Self::builder_fixed] instead.
    fn builder() -> ScreenScopeBuilder<Self> {
        ScreenScopeBuilder::<Self>::default()
    }
    /// Creates a new [ScreenScopeBuilder] which will run in [FixedMain] after
    /// [FixedUpdate]. To run on a non-fixed schedule, use [Self::builder] instead.
    fn builder_fixed() -> ScreenScopeBuilder<Self> {
        ScreenScopeBuilder::<Self>::fixed()
    }
}

#[derive(Deref)]
pub struct ScreenWrapper<T: Screen>(pub T);
impl<T> Component for ScreenWrapper<T>
where
    T: Screen,
{
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Immutable;

    fn on_add() -> Option<ComponentHook> {
        info!("ScreenWrapper::on_add");
        Some(|mut world: DeferredWorld, ctx: HookContext| {
            T::init(&mut world, &ctx);
            world.commands().entity(ctx.entity).observe(T::on_unload);
        })
    }
}

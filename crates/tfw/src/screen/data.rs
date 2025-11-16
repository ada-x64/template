use crate::prelude::*;
use bevy::{
    ecs::{component::ComponentId, schedule::ScheduleLabel, system::SystemId},
    platform::collections::HashMap,
};
use std::marker::PhantomData;

/// Triggered when a [Screen] finishes unloading and is
/// ready to transition.
#[derive(Event, Debug, PartialEq, Eq, Clone, Copy)]
pub struct FinishUnload;

/// Call this when you want to switch screens.
#[derive(Event, Debug, PartialEq, Eq, Clone, Deref, Default)]
pub struct SwitchToScreen<S: Screen>(PhantomData<S>);

/// Marker struct for a screen.
#[derive(Component, Reflect)]
pub struct ScreenMarker(pub ComponentId);

pub struct ScreenInfo {
    pub id: ComponentId,
    pub spawn: SystemId,
}

/// Stores a map from the system's name to its spawn function.
/// Used to dynamically load a screen.
#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub struct ScreenRegistry(HashMap<String, SystemId>);

/// An empty settings parameter.
#[derive(Resource, Default)]
pub struct NoSettings;

/// An empty [AssetCollection]. Combine this with the Nonblocking
/// [LoadingStrategy] to skip asset loading.
/// Note: This will _never_ resolve, so the [ScreenLoadingState] will _never_ be
/// Ready.
#[derive(Resource, Default, AssetCollection)]
pub struct NoAssets {}

/// A screen's [Schedule]. All systems added to this schedule, using the
/// [ScreenScope] below, will be scoped to this screen's lifetime. That is,
/// they will only run when the screen is in [ScreenStatus::Ready].
#[allow(bevy::unconventional_naming)]
#[derive(ScheduleLabel, SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenScope<T: Screen> {
    _Ghost(PhantomData<T>),
    Main,
    Fixed,
}

/// [State] for a [Screen]. This is the main state mechanism for screens. It is
/// also used as the type parameter for [LoadingState]. See the
/// [bevy_asset_loader] docs for more info on how asset loading works.
/// NOTE: Because
/// assets are unloaded when all handles are released, _and_ because unload
/// hooks are guaranteed to run within one schedule, there is no need to have a
/// Unloading step.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ScreenState<T: Screen> {
    #[default]
    Unloaded,
    Loading,
    Ready,
    _Phantom(PhantomData<T>),
}
impl<T: Screen> ScreenState<T> {
    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Ready)
    }
    pub fn is_loading(&self) -> bool {
        matches!(self, Self::Loading)
    }
    pub fn is_unloading(&self) -> bool {
        matches!(self, Self::Unloaded)
    }
    pub fn is_unloaded(&self) -> bool {
        matches!(self, Self::Unloaded)
    }
}

/// Scopes an entity to the current screen. The entity will be cleaned up when
/// the [Screens] state changes. By default, all entities _except_ those listed
/// in the [module documentation](crate::framework::screen) are screen-scoped.
///
/// Note: This is effectively used to stop the downward propagation of the
/// [Persistent] component. Since screen scoping is the default behavior, it
/// should not be necessary to add this component in other cases.
#[derive(Component, Debug, Reflect, Clone, Copy, Default, PartialEq)]
pub struct ScreenScoped;

/// Marks an entity as screen-persistent, i.e., this entity will _not_ be
/// automatically cleaned up when the screen changes. By default, all entites
/// _except_ those listed in the [module
/// documentation](crate::framework::screen) are screen-scoped.
///
/// In order to mark the children of this component as Persistent, you should
/// use the [Propagate] component.
#[derive(Component, Debug, Reflect, Clone, Copy, Default, PartialEq)]
pub struct Persistent;

/// How should the screen load its assets?
/// If `LoadingStrategy` is Blocking, the screen's systems will not run until
/// loading is complete. If it is Nonblocking, the screen's systems will run
/// regardless of asset completion status.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoadingStrategy {
    Blocking,
    Nonblocking,
}
impl LoadingStrategy {
    pub fn is_blocking(&self) -> bool {
        matches!(self, Self::Blocking)
    }
}

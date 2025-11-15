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
    Component
    + Sized
    + Default
    + std::fmt::Debug
    + Clone
    + Copy
    + Eq
    + std::hash::Hash
    + Send
    + Sync
    + 'static
{
    /// The associated settings type. Set as [EmptySettings] for no settings.
    type SETTINGS: Resource + FromWorld;
    /// Any associated assets which will load before the screen is considered
    /// ready. Use [EmptyAssetCollection] to skip loading.
    /// If you want to load in assets without blocking the scoped systems,
    /// you should include asset collections and states within a service.
    type ASSETS: AssetCollection;

    /// Used to confgure the screen. See [ScreenOptions] for more details.
    fn options() -> ScreenOptions;

    fn name() -> ScreenName {
        Self::options().name
    }

    fn strategy() -> LoadingStrategy {
        Self::options().strategy
    }

    /// Called when the screen is about to unload.
    /// Use this to perform any necessary cleanup before the screen transitions.
    fn unload() -> impl System<In = (), Out = ()> {
        IntoSystem::into_system(|| {})
    }
}

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

/// Options for the screen.
pub struct ScreenOptions {
    pub strategy: LoadingStrategy,
    pub name: ScreenName,
}

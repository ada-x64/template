use bevy::ecs::component::ComponentIdFor;

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
    + Reflect
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
    /// [LoadingStrategy] for the [Screen].
    const STRATEGY: LoadingStrategy;

    fn name() -> String {
        let default = Self::default();
        Reflect::as_reflect(&default)
            .reflect_short_type_path()
            .to_owned()
    }

    /// Gets the spawn function
    fn spawn(
        mut commands: Commands,
        mut next_state: ResMut<NextState<ScreenState<Self>>>,
        id: ComponentIdFor<Self>,
    ) {
        commands.spawn((Self::default(), Name::new(Self::name()), ScreenMarker(*id)));
        next_state.set(ScreenState::Loading);
    }
}

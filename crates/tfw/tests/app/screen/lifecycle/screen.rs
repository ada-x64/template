use crate::prelude::*;

/// This struct can be used to dynamically change the screen's behavior.
#[derive(PartialEq, Eq, Clone, Debug, Hash, Reflect, Default, Resource)]
pub struct LifecycleSettings;

#[derive(AssetCollection, Resource, Debug, Default)]
pub struct LifecycleAssets {
    #[asset(path = "test/cat2.png")]
    pub img: Handle<Image>,
}

#[derive(Resource, Debug, Default)]
pub struct LifecycleStatus {
    pub loading: bool,
    pub ready: bool,
    // pub unloading: bool,
    pub unloaded: bool,
    pub in_init: bool,
    pub in_unload: bool,
}

/// The main [Screen] implementation.
#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Reflect)]
pub struct LifecycleScreen;
impl Screen for LifecycleScreen {
    type SETTINGS = LifecycleSettings;
    type ASSETS = LifecycleAssets;
    const STRATEGY: LoadingStrategy = LoadingStrategy::Blocking;
}

fn init(mut r: ResMut<LifecycleStatus>) {
    r.in_init = true;
}

fn unload(mut r: ResMut<LifecycleStatus>) {
    r.in_unload = true;
}

pub fn plugin(app: &mut App) {
    ScreenScopeBuilder::<LifecycleScreen>::new(app)
        .on_ready(init)
        .on_unload(unload)
        .build();
    app.init_resource::<LifecycleStatus>();
}

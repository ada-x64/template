use crate::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug, Hash, Reflect, Default, Resource)]
pub struct BlockingScopedSystemSettings {
    pub initial_value: u32,
    pub unload_value: u32,
}
#[derive(PartialEq, Eq, Clone, Debug, Hash, Reflect, Default, Resource, Deref, DerefMut)]
pub struct BlockingScopedSystemValue(pub u32);

#[derive(AssetCollection, Resource, Debug, Default)]
pub struct BlockingScopedSystemAssets {
    #[asset(path = "test/cat2.png")]
    img: Handle<Image>,
}

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Reflect)]
pub struct BlockingScopedSystemScreen;
impl Screen for BlockingScopedSystemScreen {
    type SETTINGS = BlockingScopedSystemSettings;
    type ASSETS = BlockingScopedSystemAssets;
    const STRATEGY: LoadingStrategy = LoadingStrategy::Blocking;
}

fn increment(mut value: ResMut<BlockingScopedSystemValue>) {
    **value += 1;
}

fn init(mut value: ResMut<BlockingScopedSystemValue>, settings: Res<BlockingScopedSystemSettings>) {
    **value = settings.initial_value;
}
fn unload(
    mut value: ResMut<BlockingScopedSystemValue>,
    settings: Res<BlockingScopedSystemSettings>,
) {
    **value = settings.unload_value;
}

pub fn plugin(app: &mut App) {
    ScreenScopeBuilder::<BlockingScopedSystemScreen>::new(app)
        .add_systems(increment)
        .on_ready(init)
        .on_unload(unload)
        .build();
    app.init_resource::<BlockingScopedSystemValue>();
}

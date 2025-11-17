use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Reflect)]
pub struct EmptyScreen;
impl Screen for EmptyScreen {
    type SETTINGS = NoSettings;
    type ASSETS = NoAssets;
    const STRATEGY: LoadingStrategy = LoadingStrategy::Nonblocking;
}

pub fn plugin(app: &mut App) {
    ScreenScopeBuilder::<EmptyScreen>::new(app).build();
}

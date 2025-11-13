use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct EmptyScreen;
impl Screen for EmptyScreen {
    type SETTINGS = EmptySettings;
    type ASSETS = EmptyAssetCollection;
    fn name() -> ScreenType {
        Screens::Empty.into()
    }
}

pub fn plugin(app: &mut App) {
    debug!("in test plugin");
    ScreenScopeBuilder::<EmptyScreen>::new(app).build();
}

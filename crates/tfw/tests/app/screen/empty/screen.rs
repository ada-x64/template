use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct EmptyScreen;
impl Screen for EmptyScreen {
    type SETTINGS = EmptySettings;
    type ASSETS = NoAssets;
    fn options() -> ScreenOptions {
        ScreenOptions {
            name: Screens::Empty.into(),
            strategy: LoadingStrategy::Nonblocking,
        }
    }
}

pub fn plugin(app: &mut App) {
    debug!("in test plugin");
    ScreenScopeBuilder::<EmptyScreen>::new(app).build();
}

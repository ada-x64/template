use crate::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug, Hash, Reflect, Default, Resource)]
pub struct NamedEntityScreenSettings {
    pub entity_name: String,
}

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct NamedEntityScreen;
impl Screen for NamedEntityScreen {
    type SETTINGS = NamedEntityScreenSettings;
    type ASSETS = EmptyAssetCollection;
    fn name() -> ScreenType {
        Screens::NamedEntity.into()
    }
}

fn on_ready(settings: Res<NamedEntityScreenSettings>, mut commands: Commands) {
    debug!("on enter (Test)");
    commands.spawn(Name::new(settings.entity_name.clone()));
}

pub fn plugin(app: &mut App) {
    ScreenScopeBuilder::<NamedEntityScreen>::new(app)
        .on_ready(on_ready)
        .build();
}

use crate::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug, Hash, Reflect, Default, Resource)]
pub struct ScopedSystemSettings {
    pub value: u32,
}

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ScopedSystemScreen;
impl Screen for ScopedSystemScreen {
    type SETTINGS = ScopedSystemSettings;
    type ASSETS = NoAssets;
    fn options() -> ScreenOptions {
        ScreenOptions {
            name: Screens::ScopedSystem.into(),
            strategy: LoadingStrategy::Nonblocking,
        }
    }
}

fn init(settings: Res<ScopedSystemSettings>, mut value: ResMut<ScopedSystemValue>) {
    info!("In ScopedSystemScreen");
    **value = settings.value;
}

pub fn plugin(app: &mut App) {
    ScreenScopeBuilder::<ScopedSystemScreen>::new(app)
        .add_systems(scoped_service_systems().take())
        .on_ready(init)
        .build();
}

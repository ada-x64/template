use crate::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug, Hash, Reflect, Default, Resource)]
pub struct ScopedSystemSettings {
    pub value: u32,
}

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ScopedSystemScreen;
impl Screen for ScopedSystemScreen {
    type SETTINGS = ScopedSystemSettings;

    fn name() -> ScreenType {
        Screens::ScopedSystem.into()
    }

    fn init<'w>(mut world: DeferredWorld<'w>, _ctx: HookContext) {
        info!("In ScopedSystemScreen");
        let value = world.resource::<ScopedSystemSettings>().value;
        let mut res = world.resource_mut::<ScopedSystemValue>();
        **res = value;
    }
}

pub fn plugin(app: &mut App) {
    ScreenScopeBuilder::<ScopedSystemScreen>::default()
        .add_systems(scoped_service_systems().take())
        .build(app);
}

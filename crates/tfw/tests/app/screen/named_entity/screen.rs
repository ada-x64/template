use crate::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug, Hash, Reflect, Default, Resource)]
pub struct NamedEntityScreenSettings {
    pub entity_name: String,
}

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[component(on_insert = init, on_remove = deinit)]
pub struct NamedEntityScreen;
impl Screen for NamedEntityScreen {
    type SETTINGS = NamedEntityScreenSettings;
    type ASSETS = NoAssets;
    fn options() -> ScreenOptions {
        ScreenOptions {
            name: Screens::NamedEntity.into(),
            strategy: LoadingStrategy::Nonblocking,
        }
    }
}

fn init<'w>(mut world: DeferredWorld<'w>, _ctx: HookContext) {
    debug!("on_insert");
    let name = world
        .resource::<NamedEntityScreenSettings>()
        .entity_name
        .clone();
    world.commands().spawn(Name::new(name));
}

fn deinit<'w>(_world: DeferredWorld<'w>, _ctx: HookContext) {
    debug!("deinit")
}

pub fn plugin(app: &mut App) {
    ScreenScopeBuilder::<NamedEntityScreen>::new(app).build();
}

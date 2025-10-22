use bevy::reflect::{DynamicEnum, DynamicTuple, Enum};

use crate::prelude::*;

/// Begins unloading the current screen. Sets the NextScreen.
/// When the current screen finishes unloading, it will
fn on_switch_screen(
    trigger: Trigger<SwitchScreen>,
    mut next_screen: ResMut<NextScreen>,
    state: Res<State<Screens>>,
    mut next_state: ResMut<NextState<Screens>>,
) {
    info!("on_switch_screen");
    let mut tup = DynamicTuple::default();
    tup.insert(ScreenStatus::Unloading);
    let dy = DynamicEnum::new(state.variant_name(), tup);
    next_state.set(Screens::from_reflect(&dy).unwrap());
    *next_screen = NextScreen(Some(trigger.screen()));
}

fn on_finish_unload(
    _trigger: Trigger<FinishUnload>,
    mut next_screen: ResMut<NextScreen>,
    mut next_state: ResMut<NextState<Screens>>,
    screen_scoped: Query<Entity, With<ScreenScoped>>,
    mut commands: Commands,
) {
    info!("on_finish_unload");
    next_state.set(next_screen.0.take().unwrap());
    for entity in screen_scoped {
        commands.entity(entity).despawn();
    }
}

pub fn plugin(app: &mut App) {
    app.add_observer(on_switch_screen);
    app.add_observer(on_finish_unload);
}

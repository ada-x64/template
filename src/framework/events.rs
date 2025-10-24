use crate::prelude::*;

fn on_switch_screen(
    trigger: Trigger<SwitchScreen>,
    mut next_screen: ResMut<NextScreen>,
    state: Res<State<CurrentScreen>>,
    mut next_state: ResMut<NextState<CurrentScreen>>,
) {
    info!("on_switch_screen");
    next_state.set(state.unloading());
    *next_screen = NextScreen(Some(trigger.0));
}

fn on_finish_unload(
    _trigger: Trigger<FinishUnload>,
    mut next_screen: ResMut<NextScreen>,
    mut next_state: ResMut<NextState<CurrentScreen>>,
    screen_scoped: Query<Entity, With<ScreenScoped>>,
    mut commands: Commands,
) {
    info!("on_finish_unload");
    next_state.set(CurrentScreen {
        screen: next_screen.0.take().unwrap(),
        status: ScreenStatus::Loading,
    });
    for entity in screen_scoped {
        commands.entity(entity).despawn();
    }
}

pub fn plugin(app: &mut App) {
    app.add_observer(on_switch_screen);
    app.add_observer(on_finish_unload);
}

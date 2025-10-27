use crate::prelude::*;

fn on_switch_screen(
    trigger: Trigger<SwitchToScreen>,
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
    // Any entity which is (explicitly marked as ScreenScoped, or is _not_ marked
    // as persistent) _and_ is not a top-level observer
    screen_scoped: Query<
        Entity,
        (
            Or<(
                With<ScreenScoped>,  // is explicitly screen-scoped
                Without<Persistent>, // is explicitly persistent
            )>,
            Not<(With<Observer>, Without<ChildOf>)>, // top-level observer
        ),
    >,
    mut commands: Commands,
) {
    info!("on_finish_unload");
    next_state.set(CurrentScreen {
        screen: next_screen.0.take().unwrap(),
        status: ScreenStatus::Loading,
    });

    screen_scoped.iter().for_each(|e| {
        commands.entity(e).despawn();
    });
}

pub fn plugin(app: &mut App) {
    app.add_observer(on_switch_screen);
    app.add_observer(on_finish_unload);
}

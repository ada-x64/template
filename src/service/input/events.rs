use crate::prelude::*;
use bevy::window::CursorGrabMode;

fn exit_app(_: Trigger<Completed<PAQuit>>, mut commands: Commands, win: Single<&Window>) {
    info!("exit_app");
    if matches!(win.cursor_options.grab_mode, CursorGrabMode::None) {
        commands.send_event(AppExit::Success);
    }
}

pub fn plugin(app: &mut App) {
    app.add_observer(exit_app);
}

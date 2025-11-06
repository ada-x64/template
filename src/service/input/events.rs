use crate::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

fn exit_app(
    _: On<Complete<PAQuit>>,
    mut commands: Commands,
    cursor: Single<&CursorOptions, With<PrimaryWindow>>,
) {
    debug!("exit_app");
    if matches!(cursor.grab_mode, CursorGrabMode::None) {
        commands.write_message(AppExit::Success);
    }
}

fn spawn_global_ctx(_: On<SpawnGlobalCtx>, mut commands: Commands) {
    commands.spawn((
        ICtxGlobal,
        ContextActivity::<ICtxGlobal>::ACTIVE,
        ContextPriority::<ICtxGlobal>::new(1000),
        actions![
            ICtxGlobal[(
                Action::<PAQuit>::new(),
                bindings![KeyCode::Escape],
                ActionSettings {
                    consume_input: false,
                    require_reset: true,
                    ..Default::default()
                }
            )]
        ],
    ));
}

pub fn plugin(app: &mut App) {
    app.add_observer(exit_app);
    app.add_observer(spawn_global_ctx);
}

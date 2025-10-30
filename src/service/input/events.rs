use crate::prelude::*;
use bevy::window::CursorGrabMode;

fn exit_app(_: Trigger<Completed<PAQuit>>, mut commands: Commands, win: Single<&Window>) {
    debug!("exit_app");
    if matches!(win.cursor_options.grab_mode, CursorGrabMode::None) {
        commands.send_event(AppExit::Success);
    }
}

fn spawn_global_ctx(_: Trigger<SpawnGlobalCtx>, mut commands: Commands) {
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

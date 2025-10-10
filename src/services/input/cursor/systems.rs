use crate::prelude::*;

fn spawn_capture_cursor_actions(mut commands: Commands) {
    // info!("spawn_capture_cursor_actions");
    commands.spawn((
        Name::new("Cursor capture"),
        ICtxCaptureCursor,
        ContextActivity::<ICtxCaptureCursor>::ACTIVE,
        // todo: state scope?
        actions![
            ICtxCaptureCursor[
                (
                    Action::<PACaptureCursor>::new(),
                    bindings![MouseButton::Left]
                ),
                (
                    Action::<PAReleaseCursor>::new(),
                    bindings![KeyCode::Escape],
                    ActionSettings {
                        consume_input: true,
                        require_reset: true,
                        ..Default::default()
                    }
                ),
           ]
        ],
    ));
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_capture_cursor_actions);
}

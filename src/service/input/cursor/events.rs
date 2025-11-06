use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::prelude::*;

fn on_capture_cursor(
    _: On<Complete<PACaptureCursor>>,
    mut cursor: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut commands: Commands,
    ictx_cam_default: Query<
        // TODO: Replace with generic CameraController so we can toggle controllers separately from views
        (Entity, &Camera),
        (
            With<ContextActivity<ICtxTrackingCam>>,
            Without<ContextActivity<FlyCam>>,
        ),
    >,
    ictx_flycam: Query<
        (Entity, &Camera),
        (
            With<ContextActivity<FlyCam>>,
            Without<ContextActivity<ICtxTrackingCam>>,
        ),
    >,
) {
    cursor.visible = false;
    cursor.grab_mode = CursorGrabMode::Locked;
    {
        // switch based on active camera
        if let Ok((ictx, cam)) = ictx_cam_default.single() {
            commands
                .entity(ictx)
                .insert(ContextActivity::<ICtxTrackingCam>::new(cam.is_active));
        }
        if let Ok((ictx, cam)) = ictx_flycam.single() {
            commands
                .entity(ictx)
                .insert(ContextActivity::<FlyCam>::new(cam.is_active));
        }
    }
}
fn on_release_cursor(
    _: On<Complete<PAReleaseCursor>>,
    mut cursor: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut commands: Commands,
    ictx_cam_default: Query<Entity, With<ContextActivity<ICtxTrackingCam>>>,
    #[cfg(feature = "dev")] ictx_flycam: Query<Entity, With<ContextActivity<FlyCam>>>,
) {
    debug!("release_mouse");
    cursor.visible = true;
    cursor.grab_mode = CursorGrabMode::None;
    if let Ok(ictx_default) = ictx_cam_default.single() {
        commands
            .entity(ictx_default)
            .insert(ContextActivity::<ICtxTrackingCam>::INACTIVE);
    }
    #[cfg(feature = "dev")]
    {
        if let Ok(ictx_flycam) = ictx_flycam.single() {
            commands
                .entity(ictx_flycam)
                .insert(ContextActivity::<FlyCam>::INACTIVE);
        }
    }
}

fn spawn_cursor_capture(_trigger: On<SpawnCursorCapture>, mut commands: Commands) {
    debug!("spawn_capture_cursor_actions");
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
    app.add_observer(on_capture_cursor)
        .add_observer(on_release_cursor)
        .add_observer(spawn_cursor_capture);
}

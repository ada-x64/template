use bevy::window::CursorGrabMode;

use crate::prelude::*;

fn on_capture_cursor(
    _: Trigger<Completed<PACaptureCursor>>,
    mut window: Single<&mut Window>,
    mut commands: Commands,
    ictx_cam_default: Query<
        // TODO: Replace with generic CameraController so we can toggle controllers separately from views
        (Entity, &Camera),
        (
            With<ContextActivity<ICtxTrackingCam>>,
            Without<ContextActivity<FlyCam>>,
        ),
    >,
    #[cfg(feature = "dev")] ictx_flycam: Query<
        (Entity, &Camera),
        (
            With<ContextActivity<FlyCam>>,
            Without<ContextActivity<ICtxTrackingCam>>,
        ),
    >,
) {
    window.cursor_options.visible = false;
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
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
    _: Trigger<Completed<PAReleaseCursor>>,
    mut window: Single<&mut Window>,
    mut commands: Commands,
    ictx_cam_default: Query<Entity, With<ContextActivity<ICtxTrackingCam>>>,
    #[cfg(feature = "dev")] ictx_flycam: Query<Entity, With<ContextActivity<FlyCam>>>,
) {
    // info!("release_mouse");
    window.cursor_options.visible = true;
    window.cursor_options.grab_mode = CursorGrabMode::None;
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

pub fn plugin(app: &mut App) {
    app.add_observer(on_capture_cursor)
        .add_observer(on_release_cursor);
}

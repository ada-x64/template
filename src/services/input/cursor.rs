// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use bevy::window::CursorGrabMode;

use super::data::*;
use crate::prelude::*;

fn on_capture_cursor(
    _: Trigger<Completed<PACaptureCursor>>,
    mut window: Single<&mut Window>,
    mut commands: Commands,
    ictx_cam_default: Query<
        // TODO: Replace with generic CameraController so we can toggle controllers separately from views
        (Entity, &Camera),
        (
            With<ContextActivity<ICtxCamDefault>>,
            Without<ContextActivity<FlyCam>>,
        ),
    >,
    #[cfg(feature = "dev")] ictx_flycam: Query<
        (Entity, &Camera),
        (
            With<ContextActivity<FlyCam>>,
            Without<ContextActivity<ICtxCamDefault>>,
        ),
    >,
) {
    info!("grab_mouse");
    window.cursor_options.visible = false;
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
    #[cfg(feature = "dev")]
    {
        // switch based on active camera
        if let Ok((ictx, cam)) = ictx_cam_default.single() {
            commands
                .entity(ictx)
                .insert(ContextActivity::<ICtxCamDefault>::new(cam.is_active));
        }
        if let Ok((ictx, cam)) = ictx_flycam.single() {
            commands
                .entity(ictx)
                .insert(ContextActivity::<FlyCam>::new(cam.is_active));
        }
    }
    #[cfg(not(feature = "dev"))]
    {
        commands
            .entity(ictx_cam_default.0)
            .insert(ContextActivity::<ICtxCamDefault>::ENABLED);
    }
}
fn on_release_cursor(
    _: Trigger<Completed<PAReleaseCursor>>,
    mut window: Single<&mut Window>,
    mut commands: Commands,
    ictx_cam_default: Query<Entity, With<ContextActivity<ICtxCamDefault>>>,
    #[cfg(feature = "dev")] ictx_flycam: Query<Entity, With<ContextActivity<FlyCam>>>,
) {
    info!("release_mouse");
    window.cursor_options.visible = true;
    window.cursor_options.grab_mode = CursorGrabMode::None;
    if let Ok(ictx_default) = ictx_cam_default.single() {
        commands
            .entity(ictx_default)
            .insert(ContextActivity::<ICtxCamDefault>::INACTIVE);
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

fn spawn_capture_cursor_actions(mut commands: Commands) {
    info!("spawn_capture_cursor_actions");
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
    app.add_input_context::<ICtxCaptureCursor>()
        .add_observer(on_capture_cursor)
        .add_observer(on_release_cursor)
        .add_systems(Startup, spawn_capture_cursor_actions);
}

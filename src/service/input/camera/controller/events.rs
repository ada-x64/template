use crate::prelude::*;

fn update_camera_controller(
    trigger: Trigger<InsertCameraController>,
    mut commands: Commands,
    mut camera: Query<&mut Camera>,
) {
    commands
        .entity(trigger.entity)
        .insert(trigger.new_controller);
    let mut cam = camera
        .get_mut(trigger.entity)
        .expect("No camera to be controlled!");
    cam.is_active = trigger.new_controller.active;
    match trigger.new_controller.kind {
        CameraControllerKind::Fly => {
            info!("ICtxFlyCam => {}", trigger.new_controller.enabled);
            commands
                .entity(trigger.entity)
                .insert(ContextActivity::<ICtxFlyCam>::new(
                    trigger.new_controller.enabled,
                ));
        }
        CameraControllerKind::Tracking => {
            info!("ICtxTrackingCam => {}", trigger.new_controller.enabled);
            commands
                .entity(trigger.entity)
                .insert(ContextActivity::<ICtxTrackingCam>::new(
                    trigger.new_controller.enabled,
                ));
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_observer(update_camera_controller);
}

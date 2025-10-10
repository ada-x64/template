use std::f32::consts::{FRAC_PI_2, FRAC_PI_8};

use crate::prelude::*;

fn on_move(trigger: Trigger<Fired<PAMoveCam>>, mut transforms: Query<&mut Transform>) {
    let mut transform = r!(transforms.get_mut(trigger.target()));

    // Move to the camera direction.
    let rotation = transform.rotation.to_euler(EulerRot::YXZ);

    // Movement consists of X and -Z components, so swap Y and Z with negation.
    // We could do it with modifiers, but it wold be weird for an action to return
    // a `Vec3` like this, so we doing it inside the function.
    let mut movement = trigger.value.extend(0.0).xzy();
    movement.z = -movement.z;

    transform.translation += Quat::from_euler(EulerRot::YXZ, rotation.0, 0., 0.) * movement
}

fn on_move_y(trigger: Trigger<Fired<PAMoveCamY>>, mut transforms: Query<&mut Transform>) {
    let mut tf = r!(transforms.get_mut(trigger.target()));
    *tf = tf.with_translation(tf.translation.with_y(tf.translation.y + trigger.value));
}

fn rotate(
    trigger: Trigger<Fired<PARotateCam>>,
    mut transforms: Query<&mut Transform>,
    window: Single<&Window>,
) {
    if window.cursor_options.visible {
        return;
    }

    let mut transform = transforms.get_mut(trigger.target()).unwrap();
    let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

    yaw += trigger.value.x.to_radians();
    pitch += trigger.value.y.to_radians();

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
}

fn zoom(trigger: Trigger<Fired<PAZoomCam>>, mut projections: Query<&mut Projection>) {
    let mut projection = projections.get_mut(trigger.target()).unwrap();
    let Projection::Perspective(projection) = &mut *projection else {
        panic!("camera should be perspective");
    };
    projection.fov = (projection.fov - trigger.value).clamp(FRAC_PI_8, FRAC_PI_2);
}

pub fn plugin(app: &mut App) {
    app.add_observer(on_move)
        .add_observer(on_move_y)
        .add_observer(rotate)
        .add_observer(zoom);
}

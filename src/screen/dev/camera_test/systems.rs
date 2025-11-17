use crate::prelude::*;

fn update(mut query: Query<&mut Transform, With<Cube>>, time: Res<Time>) {
    let mut tf = r!(query.single_mut());
    *tf = tf.with_translation(Vec3::new(
        3. * f32::cos(time.elapsed_secs()) - 1.5,
        1.,
        3. * f32::sin(time.elapsed_secs()) - 1.5,
    ));
}

/// Systems for the camera_test screen.
pub fn systems() -> ServiceSystems {
    ServiceSystems::new(update)
}

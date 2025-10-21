use crate::prelude::*;

fn render_camera_gizmos(
    mut cam_gizmos: Gizmos<CameraGizmoConfigGroup>,
    config_store: Res<GizmoConfigStore>,
    fly_cam: Query<&Transform, With<FlyCam>>,
    player_cam: Query<(&Transform, &TrackingCam), With<TrackingCam>>,
    player_tf: Query<&Transform, With<PlayerController>>,
) {
    let config = config_store.config::<CameraGizmoConfigGroup>().1;
    if let Ok(tf) = fly_cam.single() {
        cam_gizmos.sphere(tf.to_isometry(), config.radius, config.fly_cam_color);
    }
    if let Ok((tf, controller)) = player_cam.single() {
        cam_gizmos.sphere(tf.to_isometry(), config.radius, config.player_cam_color);
        if let Ok(tf) = player_tf.single()
            && config.render_player_cam_spheres
        {
            // cam_gizmos.sphere(
            //     tf.to_isometry(),
            //     controller.inner_radius,
            //     config.player_cam_sphere_color,
            // );
            cam_gizmos.sphere(
                tf.to_isometry(),
                controller.outer_radius,
                config.player_cam_sphere_color,
            );
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, render_camera_gizmos);
}

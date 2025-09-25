// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use avian3d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::{color::palettes::css::*, prelude::*, render::view::RenderLayers};

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(PhysicsDebugPlugin::default())
        .insert_gizmo_config(
            PhysicsGizmos::default(),
            GizmoConfig {
                render_layers: RenderLayers::from(RenderLayer::GIZMOS_3D),
                enabled: true,
                ..Default::default()
            },
        )
        .insert_gizmo_config(
            LightGizmoConfigGroup::default(),
            GizmoConfig {
                render_layers: RenderLayers::from(RenderLayer::GIZMOS_3D),
                enabled: false,
                ..Default::default()
            },
        )
        .insert_gizmo_config(
            CameraGizmoConfigGroup::default(),
            GizmoConfig {
                render_layers: RenderLayers::from(RenderLayer::GIZMOS_3D),
                enabled: true,
                line: GizmoLineConfig {
                    style: GizmoLineStyle::Dashed {
                        gap_scale: 1.,
                        line_scale: 1.,
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .add_systems(Update, render_camera_gizmos);
}

#[derive(GizmoConfigGroup, Reflect)]
pub struct CameraGizmoConfigGroup {
    pub radius: f32,
    pub fly_cam_color: Color,
    pub player_cam_color: Color,
    pub render_player_cam_spheres: bool,
    pub player_cam_sphere_color: Color,
}
impl Default for CameraGizmoConfigGroup {
    fn default() -> Self {
        Self {
            radius: 1.,
            fly_cam_color: RED.into(),
            player_cam_color: LIGHT_BLUE.into(),
            render_player_cam_spheres: true,
            player_cam_sphere_color: YELLOW.into(),
        }
    }
}

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
            cam_gizmos.sphere(
                tf.to_isometry(),
                controller.inner_radius,
                config.player_cam_sphere_color,
            );
            cam_gizmos.sphere(
                tf.to_isometry(),
                controller.outer_radius,
                config.player_cam_sphere_color,
            );
        }
    }
}

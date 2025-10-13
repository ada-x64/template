use bevy::{color::palettes::css::*, prelude::*};

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

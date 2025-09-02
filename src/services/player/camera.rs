use bevy::{prelude::*, render::view::RenderLayers};

use crate::{
    data::{CameraOrder, RenderLayer},
    screens::ScreenStates,
};

/// Camera which tracks the player.
#[derive(Component, Debug, Default)]
pub struct PlayerCam;

/// Spawns the gameplay camera
pub fn player_cam() -> impl Bundle {
    (
        PlayerCam,
        StateScoped(ScreenStates::InWorld),
        Name::new("PlayerCam"),
        Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera3d::default(),
        PointLight::default(),
        Camera {
            order: CameraOrder::Player.into(),
            clear_color: ClearColorConfig::Custom(
                bevy::color::palettes::tailwind::SLATE_800.into(), // just to ensure it's actually rendering
            ),
            ..Default::default()
        },
    )
}

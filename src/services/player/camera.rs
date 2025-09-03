use bevy::prelude::*;

use crate::services::player::data::*;

/// Spawns the gameplay camera
pub fn track_player(
    pt: Single<&Transform, (With<PlayerController>, Without<PlayerCam>)>,
    mut ct: Single<&mut Transform, (With<PlayerCam>, Without<PlayerRoot>)>,
) {
    **ct = ct
        .looking_at(pt.translation, Vec3::Y)
        .with_translation(pt.translation - Vec3::new(0., -5., -10.));
}

use bevy::prelude::*;
use bevy_flycam::FlyCam;
use bevy_minibuffer::prelude::*;

use crate::services::player::camera::PlayerCam;

fn toggle_flycam(
    mut set: ParamSet<(
        Single<&mut Camera, With<FlyCam>>,
        Option<Single<&mut Camera, With<PlayerCam>>>,
    )>,
    mut minibuffer: Minibuffer,
) {
    let prev_state = set.p0().is_active;
    set.p0().is_active = !prev_state;
    if let Some(mut cam) = set.p1() {
        cam.is_active = prev_state;
    }
    if prev_state {
        minibuffer.message("Returning to main camera.")
    } else {
        minibuffer.message("Using dev flycam.")
    }
}

pub fn plugin(app: &mut App) {
    app.add_acts(Act::new(toggle_flycam).bind(vec![KeyCode::Space, KeyCode::Space]));
}

// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use avian3d::prelude::PhysicsGizmos;
use bevy::{prelude::*, render::view::RenderLayers};
use bevy_flycam::FlyCam;
use bevy_minibuffer::prelude::*;

use crate::{data::RenderLayer, screens::ScreenStates, services::player::data::PlayerCam};

fn toggle_flycam(
    state: Res<State<ScreenStates>>,
    mut set: ParamSet<(
        Single<&mut Camera, With<FlyCam>>,
        Option<Single<&mut Camera, With<PlayerCam>>>,
    )>,
    mut minibuffer: Minibuffer,
) {
    if !matches!(**state, ScreenStates::InWorld) {
        set.p0().is_active = false;
        return;
    }
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

fn toggle_gizmos<T: GizmoConfigGroup>(mut g: ResMut<GizmoConfigStore>, mut minibuffer: Minibuffer) {
    let (config, _) = g.config_mut::<T>();
    config.enabled = !config.enabled;
    if config.enabled {
        minibuffer.message(format!("Showing {}", T::short_type_path()))
    } else {
        minibuffer.message(format!("Hiding {}", T::short_type_path()))
    }
}

fn spawn_flycam(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        StateScoped(ScreenStates::InWorld),
        Camera {
            is_active: false,
            ..Default::default()
        },
        FlyCam,
        Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::from(RenderLayer::DEFAULT | RenderLayer::GIZMOS_3D | RenderLayer::PARTICLES),
    ));
}

pub fn plugin(app: &mut App) {
    app.add_acts((
        Act::new(toggle_flycam).bind(vec![KeyCode::Space, KeyCode::Space]),
        // TODO could use Askyy prompts here
        Act::new(toggle_gizmos::<PhysicsGizmos>).named("toggle_physics_gizmos"),
        Act::new(toggle_gizmos::<LightGizmoConfigGroup>).named("toggle_light_gizmos"),
    ))
    .add_systems(
        OnEnter(ScreenStates::InWorld),
        spawn_flycam.run_if(|q: Query<&FlyCam>| q.is_empty()),
    );
}

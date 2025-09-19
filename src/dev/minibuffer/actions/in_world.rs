// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use avian3d::prelude::PhysicsGizmos;
use bevy::prelude::*;
use bevy_minibuffer::prelude::*;
use std::str::FromStr;
use strum::{Display, EnumString, VariantNames};

use crate::{
    dev::minibuffer::fly_cam::FlyCam, screens::ScreenStates, services::player::data::PlayerCam,
};

#[derive(Debug, Display, VariantNames, EnumString)]
enum Cameras {
    FlyCam,
    PlayerCam,
}

fn set_camera_controller() {}

fn cycle_cam() {}

fn set_cam(state: Res<State<ScreenStates>>, mut minibuffer: Minibuffer) {
    if !matches!(**state, ScreenStates::InWorld) {
        minibuffer.message("This command requires ScreenStates::InWorld");
        return;
    }
    minibuffer
        .prompt_map("Which camera?", Cameras::VARIANTS.to_vec())
        .observe(
            |mut trigger: Trigger<Submit<String>>,
             mut set: ParamSet<(
                Option<Single<&mut Camera, With<FlyCam>>>,
                Option<Single<&mut Camera, With<PlayerCam>>>,
            )>,
             mut minibuffer: Minibuffer| {
                if let Ok(val) = trigger.event_mut().take_result() {
                    match Cameras::from_str(&val).unwrap() {
                        Cameras::FlyCam => {
                            set.p0().unwrap().is_active = true;
                            set.p1().unwrap().is_active = false;
                        }
                        Cameras::PlayerCam => {
                            set.p0().unwrap().is_active = false;
                            set.p1().unwrap().is_active = true;
                        }
                    }
                    minibuffer.message(format!("Set camera to {val}"));
                } else {
                    minibuffer.message("Cancelled.");
                }
            },
        );
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

pub fn plugin(app: &mut App) {
    app.add_acts((
        Act::new(set_cam).bind(vec![KeyCode::Space, KeyCode::Space]),
        // TODO could use Askyy prompts here
        Act::new(toggle_gizmos::<PhysicsGizmos>).named("toggle_physics_gizmos"),
        Act::new(toggle_gizmos::<LightGizmoConfigGroup>).named("toggle_light_gizmos"),
    ));
}

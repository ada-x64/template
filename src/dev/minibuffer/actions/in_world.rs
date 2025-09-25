// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use avian3d::prelude::PhysicsGizmos;
use bevy::prelude::*;
use bevy_minibuffer::prelude::*;
use std::str::FromStr;
use strum::{EnumCount, VariantNames};

use crate::prelude::*;

fn set_view(state: Res<State<ScreenStates>>, mut minibuffer: Minibuffer) {
    if !matches!(**state, ScreenStates::InWorld) {
        minibuffer.message("This command requires ScreenStates::InWorld");
        return;
    }
    minibuffer
        .prompt_map("Which camera?", CameraName::VARIANTS.to_vec())
        .observe(
            |mut trigger: Trigger<Submit<String>>,
             mut commands: Commands,
             cams: Query<(Entity, &CameraName, &CameraController)>| {
                let name = r!(trigger
                    .take_result()
                    .ok()
                    .and_then(|res| CameraName::from_str(&res).ok()));

                for (entity, cam_name, controller) in cams {
                    commands.trigger(InsertCameraController {
                        entity,
                        new_controller: CameraController {
                            active: name == *cam_name,
                            ..*controller
                        },
                    });
                }
            },
        );
}

fn set_cam_controller(state: Res<State<ScreenStates>>, mut minibuffer: Minibuffer) {
    if !matches!(**state, ScreenStates::InWorld) {
        minibuffer.message("This command requires ScreenStates::InWorld");
        return;
    }
    minibuffer
        .prompt_map("Which camera?", CameraName::VARIANTS.to_vec())
        .observe(
            |mut trigger: Trigger<Submit<String>>,
             mut commands: Commands,
             cams: Query<(Entity, &CameraName, &CameraController)>| {
                let name = r!(trigger
                    .take_result()
                    .ok()
                    .and_then(|res| CameraName::from_str(&res).ok()));

                for (entity, cam_name, controller) in cams {
                    commands.trigger(InsertCameraController {
                        entity,
                        new_controller: CameraController {
                            enabled: name == *cam_name,
                            ..*controller
                        },
                    });
                }
            },
        );
}

fn cycle_view(mut commands: Commands, cams: Query<(Entity, &CameraName, &CameraController)>) {
    // get the currently active view. this could be more efficient if it were a separate component.
    let (active_entt, active_cam, active_controller) =
        r!(cams.iter().find(|(_, _, controller)| controller.active));
    let next_cam = CameraName::from_repr(*active_cam as usize + 1 % CameraName::COUNT).unwrap();
    let (next_entt, _, next_controller) = r!(cams.iter().find(|(_, name, _)| **name == next_cam));

    commands.trigger(InsertCameraController {
        entity: active_entt,
        new_controller: CameraController {
            active: false,
            ..*active_controller
        },
    });
    commands.trigger(InsertCameraController {
        entity: next_entt,
        new_controller: CameraController {
            active: true,
            ..*next_controller
        },
    })
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
        Act::new(set_view),
        Act::new(set_cam_controller),
        Act::new(cycle_view).bind(vec![KeyCode::Space, KeyCode::Space]),
        // TODO could use Askyy prompts here
        Act::new(toggle_gizmos::<PhysicsGizmos>).named("toggle_physics_gizmos"),
        Act::new(toggle_gizmos::<LightGizmoConfigGroup>).named("toggle_light_gizmos"),
    ));
}

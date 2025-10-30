use avian3d::prelude::PhysicsGizmos;

use crate::prelude::*;

macro_rules! cam_cmd {
    ($func:ident, $property:ident) => {
        fn $func(
            state: Res<State<CurrentScreen>>,
            mut minibuffer: Minibuffer,
            cam_list: Res<CameraList>,
            cams: Query<(&Name, &CameraController)>,
        ) {
            if !matches!(***state, Screens::World) {
                minibuffer.message("This command requires Screens::InWorld");
                return;
            }

            let names = cam_list
                .iter()
                .filter_map(|cam| cams.get(*cam).map(|(name, _)| name.clone()).ok())
                .collect::<Vec<_>>();

            minibuffer.prompt_map("Which camera?", names).observe(
                |mut trigger: Trigger<Submit<String>>,
                 mut commands: Commands,
                 cam_list: Res<CameraList>,
                 cams: Query<(&Name, &CameraController)>| {
                    let target_name = r!(trigger.take_result());
                    cam_list.iter().for_each(|entity| {
                        if let Ok((name, controller)) = cams.get(*entity) {
                            commands.trigger(InsertCameraController {
                                entity: *entity,
                                new_controller: CameraController {
                                    $property: **name == target_name,
                                    ..*controller
                                },
                            });
                        };
                    });
                },
            );
        }
    };
}
cam_cmd!(set_view, active);
cam_cmd!(set_cam_controller, enabled);

fn cycle_cam(
    mut commands: Commands,
    cams: Res<CameraList>,
    mut res_active_cam: ResMut<ActiveCamera>,
    controllers: Query<&CameraController>,
) {
    debug!("Cycling cameras. Current idx: {res_active_cam:?}");
    let active_cam = **res_active_cam;
    let active_controller = r!(controllers.get(cams[active_cam]));
    commands.trigger(InsertCameraController {
        entity: cams[active_cam],
        new_controller: CameraController {
            active: false,
            enabled: false,
            ..*active_controller
        },
    });
    let next_cam = (active_cam + 1) % cams.len();
    let next_controller = r!(controllers.get(cams[next_cam]));
    **res_active_cam = next_cam;
    commands.trigger(InsertCameraController {
        entity: cams[next_cam],
        new_controller: CameraController {
            active: true,
            enabled: true,
            ..*next_controller
        },
    });
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
        Act::new(cycle_cam).bind(vec![KeyCode::Space, KeyCode::Space]),
        // TODO could use Askyy prompts here
        Act::new(toggle_gizmos::<PhysicsGizmos>).named("toggle_physics_gizmos"),
        Act::new(toggle_gizmos::<LightGizmoConfigGroup>).named("toggle_light_gizmos"),
    ));
}

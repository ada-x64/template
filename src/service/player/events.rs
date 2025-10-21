use bevy_tnua_avian3d::TnuaAvian3dSensorShape;

use crate::prelude::*;

// TODO: Split this out into a bundle
fn spawn_player_root(
    _: Trigger<SpawnPlayerRoot>,
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut camera_list: ResMut<CameraList>,
) {
    let player_entt = commands
        .spawn((
            PlayerController::default(),
            StateScoped(ScreenStates::InWorld),
            SceneRoot(player_assets.model.clone()),
            (
                RigidBody::Dynamic,
                Collider::capsule(PLAYER_CAPSULE_RADIUS, PLAYER_CAPSULE_HEIGHT),
                LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
                Friction::ZERO,
            ),
            (
                TnuaController::default(),
                TnuaAvian3dSensorShape(Collider::cylinder(PLAYER_CAPSULE_RADIUS + 0.1, 0.)),
                ICtxDefault,
                ContextActivity::<ICtxDefault>::ACTIVE,
                actions!(
                    ICtxDefault[(
                        Action::<PAMove>::new(),
                        DeadZone::default(),
                        SmoothNudge::default(),
                        Scale::splat(PLAYER_DEFAULT_SPEED),
                        Negate::y(),
                        SwizzleAxis::XZY,
                        Bindings::spawn((Cardinal::wasd_keys(), Axial::left_stick())),
                    )]
                ),
            ),
        ))
        .id();

    let cam = commands
        .spawn((
            Name::new("PlayerCam"),
            StateScoped(ScreenStates::InWorld),
            (LockedAxes::new().lock_rotation_z(),),
            (
                #[cfg(feature = "dev")]
                ShowLightGizmo::default(),
                PointLight::default(),
            ),
            tracking_cam_bundle(player_entt),
        ))
        .id();
    camera_list.push(cam);
}

fn on_move(trigger: Trigger<Fired<PAMove>>, mut controller: Single<&mut PlayerController>) {
    controller.last_move = Some(trigger.value);
}

pub fn plugin(app: &mut App) {
    app.add_observer(on_move).add_observer(spawn_player_root);
}

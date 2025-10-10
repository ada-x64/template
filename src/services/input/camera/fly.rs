// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------

use core::f32::consts::{FRAC_PI_2, FRAC_PI_8};

use bevy::{prelude::*, render::view::RenderLayers};
use bevy_enhanced_input::prelude::*;

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_input_context::<FlyCam>() // All contexts should be registered.
        .add_observer(on_move)
        .add_observer(on_move_y)
        .add_observer(rotate)
        .add_observer(zoom)
        .add_systems(
            OnEnter(ScreenStates::InWorld),
            (|mut commands: Commands, mut cam_list: ResMut<CameraList>| {
                let cam = commands.spawn((Name::new("FlyCam"), flycam_bundle())).id();
                cam_list.push(cam);
            })
            .run_if(|q: Query<&FlyCam>| q.is_empty()),
        );
}

/// Don't forget to register this in the CameraList.
pub fn flycam_bundle() -> impl Bundle {
    (
        FlyCam,
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::from(RenderLayer::DEFAULT | RenderLayer::GIZMOS_3D | RenderLayer::PARTICLES),
        actions!(ICtxFlyCam[
            (
                Action::<PAMoveCam>::new(),
                DeadZone::default(),
                SmoothNudge::default(),
                Scale::splat(0.3),
                Bindings::spawn((
                    Cardinal::wasd_keys(),
                    Axial::left_stick(),
                )),
            ),
        (
            Action::<PAMoveCamY>::new(),
            Scale::splat(0.3),
            Bindings::spawn((
                Bidirectional::<Binding, Binding> {
                    positive: KeyCode::Space.into(),
                    negative: KeyCode::ShiftLeft.into(),
                },
                Bidirectional::<Binding, Binding> {
                    positive: GamepadButton::South.into(),
                    negative: GamepadButton::West.into(),
                },
            )),
        ),
            (
                Action::<PARotateCam>::new(),
                Bindings::spawn((
                    // Bevy requires single entities to be wrapped in `Spawn`.
                    // You can attach modifiers to individual bindings as well.
                    Spawn((Binding::mouse_motion(), Scale::splat(0.1), Negate::all())),
                    Axial::right_stick().with((Scale::splat(2.0), Negate::x())),
                )),
            ),
            (
                Action::<PAZoomCam>::new(),
                Scale::splat(0.1),
                Bindings::spawn((
                    // In Bevy, vertical scrolling maps to the Y axis,
                    // so we apply `SwizzleAxis` to map it to our 1-dimensional action.
                    Spawn((Binding::mouse_wheel(), SwizzleAxis::YXZ)),
                    Bidirectional::up_down_dpad(),
                )),
            ),
        ]),
    )
}

fn on_move(trigger: Trigger<Fired<PAMoveCam>>, mut transforms: Query<&mut Transform>) {
    let mut transform = r!(transforms.get_mut(trigger.target()));

    // Move to the camera direction.
    let rotation = transform.rotation.to_euler(EulerRot::YXZ);

    // Movement consists of X and -Z components, so swap Y and Z with negation.
    // We could do it with modifiers, but it wold be weird for an action to return
    // a `Vec3` like this, so we doing it inside the function.
    let mut movement = trigger.value.extend(0.0).xzy();
    movement.z = -movement.z;

    transform.translation += Quat::from_euler(EulerRot::YXZ, rotation.0, 0., 0.) * movement
}

fn on_move_y(trigger: Trigger<Fired<PAMoveCamY>>, mut transforms: Query<&mut Transform>) {
    let mut tf = r!(transforms.get_mut(trigger.target()));
    *tf = tf.with_translation(tf.translation.with_y(tf.translation.y + trigger.value));
}

fn rotate(
    trigger: Trigger<Fired<PARotateCam>>,
    mut transforms: Query<&mut Transform>,
    window: Single<&Window>,
) {
    if window.cursor_options.visible {
        return;
    }

    let mut transform = transforms.get_mut(trigger.target()).unwrap();
    let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

    yaw += trigger.value.x.to_radians();
    pitch += trigger.value.y.to_radians();

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
}

fn zoom(trigger: Trigger<Fired<PAZoomCam>>, mut projections: Query<&mut Projection>) {
    let mut projection = projections.get_mut(trigger.target()).unwrap();
    let Projection::Perspective(projection) = &mut *projection else {
        panic!("camera should be perspective");
    };
    projection.fov = (projection.fov - trigger.value).clamp(FRAC_PI_8, FRAC_PI_2);
}

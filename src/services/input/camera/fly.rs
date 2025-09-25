// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------

use core::f32::consts::{FRAC_PI_2, FRAC_PI_8};

use bevy::{prelude::*, render::view::RenderLayers};
use bevy_enhanced_input::prelude::*;

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_input_context::<FlyCam>() // All contexts should be registered.
        .add_observer(apply_movement)
        .add_observer(rotate)
        .add_observer(zoom)
        .add_systems(
            OnEnter(ScreenStates::InWorld),
            spawn_flycam.run_if(|q: Query<&FlyCam>| q.is_empty()),
        );
}

pub fn spawn_flycam(mut commands: Commands) {
    // Spawn a camera with an input context.
    commands.spawn((
        CameraName::DevCam,
        Camera3d::default(),
        StateScoped(ScreenStates::InWorld),
        Camera {
            is_active: false,
            ..Default::default()
        },
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCam,
        CameraController {
            active: false,
            enabled: false,
            kind: CameraControllerKind::Fly,
        },
        ContextActivity::<FlyCam>::INACTIVE,
        RenderLayers::from(RenderLayer::DEFAULT | RenderLayer::GIZMOS_3D | RenderLayer::PARTICLES),
        // Similar to `related!`, but you only specify the context type.
        // Actions are related to specific context since a single entity can have multiple contexts.
        actions!(FlyCam[
            (
                Action::<PAMove>::new(),
                // Conditions and modifiers as components.
                DeadZone::default(), // Apply non-uniform normalization that works for both digital and analog inputs, otherwise diagonal movement will be faster.
                SmoothNudge::default(), // Make movement smooth and independent of the framerate. To only make it framerate-independent, use `DeltaScale`.
                Scale::splat(0.3), // Additionally multiply by a constant to achieve the desired speed.
                // Bindings are entities related to actions.
                // An action can have multiple bindings and will respond to any of them.
                Bindings::spawn((
                    // Bindings like WASD or sticks are very common,
                    // so we provide built-in `SpawnableList`s to assign all keys/axes at once.
                    Cardinal::wasd_keys(),
                    Axial::left_stick(),
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
    ));
}

fn apply_movement(trigger: Trigger<Fired<PAMove>>, mut transforms: Query<&mut Transform>) {
    let mut transform = transforms.get_mut(trigger.target()).unwrap();

    // Move to the camera direction.
    let rotation = transform.rotation;

    // Movement consists of X and -Z components, so swap Y and Z with negation.
    // We could do it with modifiers, but it wold be weird for an action to return
    // a `Vec3` like this, so we doing it inside the function.
    let mut movement = trigger.value.extend(0.0).xzy();
    movement.z = -movement.z;

    transform.translation += rotation * movement
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
    projection.fov = (projection.fov + trigger.value).clamp(FRAC_PI_8, FRAC_PI_2);
}

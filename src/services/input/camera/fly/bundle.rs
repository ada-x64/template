pub use crate::prelude::*;
use bevy::render::view::RenderLayers;

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

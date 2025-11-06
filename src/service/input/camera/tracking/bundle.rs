use crate::prelude::*;

pub fn tracking_cam_bundle(tracking: Entity) -> impl Bundle {
    (
        TrackingCam::new(tracking),
        // rendering
        (
            CameraController {
                active: true,
                enabled: true,
                kind: CameraControllerKind::Tracking,
            },
            Camera {
                order: CameraOrder::World as isize,
                is_active: true,
                ..Default::default()
            },
            RenderLayers::from(
                RenderLayer::DEFAULT | RenderLayer::GIZMOS_3D | RenderLayer::PARTICLES,
            ),
        ),
        // actions
        (
            ContextActivity::<ICtxTrackingCam>::ACTIVE,
            ICtxTrackingCam,
            actions![
                ICtxTrackingCam[
                (
                    Action::<PARotateCam>::new(),
                    Bindings::spawn((
                        Axial::right_stick().with((Scale::splat(2.0), Negate::x())),
                        Spawn((Binding::mouse_motion(), Scale::splat(0.01), Negate::all()))
                    )),
                ),
                (
                    Action::<PAZoomCam>::new(),
                    DeadZone::default(),
                    SmoothNudge::default(),
                    Scale::splat(PLAYER_CAM_ZOOM_SPD),
                    Bindings::spawn(
                        (
                            Axial::right_stick(),
                            Spawn(
                                (Binding::mouse_wheel(), Scale::splat(0.1), SwizzleAxis::YXZ))
                        )
                    )
                ),
                ]
            ],
        ),
    )
}

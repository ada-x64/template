use crate::prelude::*;
use bevy::render::view::RenderLayers;
use bitflags::bitflags;

bitflags! {
    pub struct RenderLayer: u32 {
        /// Used implicitly by all entities without a `RenderLayers` component.
        /// Light source should be on DEFAULT _and_ PLAYER
        const DEFAULT = 0b00000001;
        /// For rendering first-person view
        const PLAYER = 0b00000010;
        /// Indicates this camera can render particles
        const PARTICLES = 0b00000100;
        /// Should only be rendered by a Camera3D
        const GIZMOS_3D = 0b0001000;
    }
}
impl From<RenderLayer> for RenderLayers {
    fn from(layer: RenderLayer) -> Self {
        // Render layers are just vectors of ints, so we convert each active bit to an int.
        RenderLayers::from_iter(layer.iter().map(|l| (l.bits() >> 1) as usize))
    }
}

/// Note: Value increases from top to bottom with highest value drawn last, thus
/// on top of the others
#[derive(Debug)]
pub enum CameraOrder {
    World,
    Player,
    Ui,
}

/// A global list of all the cameras in the order they should be presented
/// for the cycle_cam dev command.
#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub struct CameraList(Vec<Entity>);

/// The currently active camera view.
/// For the cycle_cam dev command.
#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub struct ActiveCamera(usize);

/// Tracking camera. Will follow the given entity. Will spawn a CameraController on add.
/// Prefer to use [tracking_cam_bundle].
#[derive(Component, Debug)]
#[require(CameraController::new(CameraControllerKind::Tracking), ICtxTrackingCam)]
pub struct TrackingCam {
    /// In radians.
    pub rotation: Vec2,
    /// radius of outer sphere. used for zoom and camera collisions.
    pub outer_radius: f32,
    /// radius of inner sphere. used for zoom and camera collisions.
    pub inner_radius: f32,
    /// Tracking entity.
    pub entity: Entity,
}
impl TrackingCam {
    pub fn new(entity: Entity) -> Self {
        Self {
            rotation: Vec2::ZERO,
            outer_radius: 10.,
            inner_radius: 1.,
            entity,
        }
    }
}

#[derive(Component)]
#[require(CameraController::new(CameraControllerKind::Fly), ICtxFlyCam)]
pub struct FlyCam;

#[derive(Component, Default)]
pub struct ICtxFlyCam;

/// Default camera input context
#[derive(Component, Default)]
pub struct ICtxTrackingCam;

/// PlayerAction_RotateCamera
#[derive(InputAction, Reflect)]
#[action_output(Vec2)]
pub struct PARotateCam;

/// PlayerAction_ZoomCamera
#[derive(InputAction, Reflect)]
#[action_output(f32)]
pub struct PAZoomCam;

/// PlayerAction_MoveCam (for [FlyCam])
#[derive(InputAction, Reflect)]
#[action_output(Vec2)]
pub struct PAMoveCam;

/// PlayerAction_MoveCamY (for [FlyCam])
#[derive(InputAction, Reflect)]
#[action_output(f32)]
pub struct PAMoveCamY;

#[derive(Debug, Reflect, Copy, Clone)]
pub enum CameraControllerKind {
    Fly,
    Tracking,
}

#[derive(Debug, Reflect, Component, Copy, Clone)]
#[require(Camera {is_active: false, ..Default::default()}, Camera3d)]
#[component(immutable)]
pub struct CameraController {
    /// Is the controller enabled?
    pub enabled: bool,
    /// Is the camera active?
    pub active: bool,
    /// What kind of camera is this?
    pub kind: CameraControllerKind,
}
impl CameraController {
    pub fn new(kind: CameraControllerKind) -> Self {
        Self {
            enabled: true,
            active: true,
            kind,
        }
    }
    pub fn with_enabled(self, enabled: bool) -> Self {
        Self { enabled, ..self }
    }
    pub fn with_active(self, active: bool) -> Self {
        Self { active, ..self }
    }
}

/// Used to update the camera controller.
#[derive(Event, Debug)]
pub struct InsertCameraController {
    pub entity: Entity,
    pub new_controller: CameraController,
}

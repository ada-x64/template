// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
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
pub enum CameraOrder {
    World,
    Player,
    Ui,
}
impl From<CameraOrder> for isize {
    fn from(order: CameraOrder) -> Self {
        order as isize
    }
}

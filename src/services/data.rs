use std::ops::BitOr;

use crate::prelude::*;
use avian3d::prelude::*;

#[derive(PhysicsLayer, Copy, Clone, Debug, Default)]
pub enum CollisionLayer {
    #[default]
    Default,
    Player,
    Camera,
}
impl BitOr for CollisionLayer {
    type Output = u32;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.to_bits() | rhs.to_bits()
    }
}

// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------

use bevy::prelude::*;

#[cfg(feature = "dev")]
pub(crate) mod dev;
pub(crate) mod services;

#[cfg(feature = "dev")]
use crate::dev::DevPlugin;

use crate::services::ServicesPlugin;

pub struct AppPlugin {}
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ServicesPlugin {});
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, AppPlugin {}));
    #[cfg(feature = "dev")]
    app.add_plugins(DevPlugin);
    // test
    app.world_mut().spawn((
        Camera3d::default(),
        Transform::from_xyz(-2., 2.5, 5.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    app.run();
}

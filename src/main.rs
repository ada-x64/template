// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------

use bevy::prelude::*;

pub(crate) mod data;
pub(crate) mod screens;
pub(crate) mod services;

#[cfg(feature = "dev")]
pub(crate) mod dev;

/// This would ideally be in lib.rs but it's here bc
/// hot patching won't work with libraries
fn app_plugins(app: &mut App) {
    app.add_plugins((services::plugin, screens::plugin));
    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);
}

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, app_plugins));
    app.run();
}

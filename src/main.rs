// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------

use bevy::prelude::*;

#[cfg(feature = "dev")]
pub(crate) mod dev;
pub(crate) mod screens;
pub(crate) mod services;

#[cfg(feature = "dev")]
pub mod dev_prelude {
    pub use bevy_minibuffer::prelude::*;
    pub use bevy_simple_subsecond_system::hot;
}
pub mod prelude {
    pub use super::screens::prelude::*;
    pub use super::services::prelude::*;

    // third party
    #[cfg(feature = "dev")]
    pub use super::dev_prelude::*;
    pub use bevy::prelude::*;
    pub use bevy_enhanced_input::prelude::Completed;
    pub use bevy_enhanced_input::prelude::*;
    pub use tiny_bail::prelude::*;
}

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

// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------

#[cfg(feature = "dev")]
pub(crate) mod dev;
pub(crate) mod screens;
pub mod services;
pub mod third_party;

#[cfg(feature = "dev")]
pub mod dev_prelude {
    pub use super::dev::plugin;
    pub use bevy_minibuffer::prelude::*;
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

pub use services::{private_plugin as __private_plugin, public_plugin as plugin};

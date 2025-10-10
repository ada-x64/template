// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------

pub mod screens;
pub mod services;

pub mod prelude {
    pub use super::screens::plugin as ScreensPlugin;
    pub use super::screens::prelude::*;
    pub use super::services::plugin as ServicesPlugin;
    pub use super::services::prelude::*;
}

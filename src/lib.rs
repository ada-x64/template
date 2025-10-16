// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------

pub mod framework;
pub mod screen;
pub mod service;

pub mod prelude {
    pub use super::framework::prelude::*;
    pub use super::screen::plugin as ScreensPlugin;
    pub use super::screen::prelude::*;
    pub use super::service::plugin as ServicesPlugin;
    pub use super::service::prelude::*;
}

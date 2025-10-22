// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------

pub mod framework;
pub mod screen;
pub mod service;

mod plugin;
pub use plugin::{AppPlugin, AppSettings};

pub mod prelude {
    pub use super::framework::prelude::*;
    pub use super::screen::prelude::*;
    pub use super::service::prelude::*;
}

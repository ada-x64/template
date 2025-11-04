//! tfw-app template.
//! For general information on how to use this template, see the [docs] module.
#[cfg(doc)]
pub mod docs;

pub mod framework;
/// See [framework::screen]
pub mod screen;
/// See [docs::architecture#modules]
pub mod service;

mod plugin;
pub use plugin::{AppPlugin, AppSettings};

pub mod prelude {
    pub use super::framework::prelude::*;
    pub use super::screen::prelude::*;
    pub use super::service::prelude::*;
}

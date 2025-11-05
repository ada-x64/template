#![feature(register_tool)]
#![register_tool(bevy)]
#![allow(bevy::panicking_methods)]

//! # tfw-app template
//!
//! This crate is split into three modules: the [screen] module for the creation
//! of screens, the [service] module for the creation of services, and the
//! [framework] module for the underlying implementation. (NOTE: Framework will
//! likely be moved to its own crate shortly.)
//!
//! For general information on how to use this template, see
//! the [docs] module.

/// Information about the general architecture and patterns used in this
/// template.
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

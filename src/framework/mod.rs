use crate::prelude::*;

/// General utility types
pub mod data;
/// Screen implementation
pub mod screen;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::data::prelude::*;
    pub use super::data::*;
    pub use super::screen::prelude::*;
    #[doc(hidden)]
    pub use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};
}

pub fn plugin(app: &mut App) {
    app.add_plugins(screen::plugin);
}

use crate::prelude::*;

mod data;
mod screen;
#[cfg(test)]
pub mod tests;
mod util;

pub mod prelude {
    pub use super::data::*;
    pub use super::screen::prelude::*;
    pub use super::util::prelude::*;
    pub use bevy::ecs::{component::HookContext, world::DeferredWorld};
}

pub fn plugin(app: &mut App) {
    app.add_plugins(screen::plugin);
}

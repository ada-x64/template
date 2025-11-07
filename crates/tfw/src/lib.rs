#![feature(register_tool)]
#![register_tool(bevy)]
#![allow(bevy::panicking_methods)]

use crate::prelude::*;

/// General utility types
pub mod data;
/// Screen implementation
pub mod screen;

#[cfg(test)]
mod test_impl;

pub mod prelude {
    pub use super::data::prelude::*;
    pub use super::data::*;
    pub use super::screen::prelude::*;
    #[cfg(test)]
    pub use super::test_impl::prelude::*;
    #[doc(hidden)]
    pub use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};
    pub(crate) use bevy::prelude::*;
}

#[derive(Resource, Debug, Reflect, Clone)]
pub struct TfwSettings {
    pub initial_screen: ScreenType,
}

/// The main export plugin for TFW. `Screens` should be an enum with screen
/// names. Refer to the template documentation for more details.
pub struct TfwPlugin {
    pub settings: TfwSettings,
}
impl Plugin for TfwPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.settings.clone());

        app.add_plugins(screen::plugin);
        #[cfg(test)]
        app.add_plugins(test_impl::plugin);
    }
}

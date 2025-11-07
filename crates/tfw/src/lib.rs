#![feature(register_tool)]
#![register_tool(bevy)]
#![allow(bevy::panicking_methods)]

use bevy::platform::collections::HashSet;

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
    #[cfg(test)]
    pub use super::tests::prelude::*;
    #[doc(hidden)]
    pub use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};
    pub(crate) use bevy::prelude::*;
}

#[derive(Resource, Default, Debug, Deref, DerefMut, Reflect)]
pub struct Screens {
    map: HashSet<ScreenType>,
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
        app.add_plugins(screen::plugin);
        app.init_resource::<Screens>();
        app.insert_resource(self.settings.clone());
        #[cfg(test)]
        app.add_plugins(tests::plugin);
    }
}

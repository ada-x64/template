// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use crate::prelude::*;

mod data;
mod world;

#[cfg(feature = "dev")]
mod dev;

pub mod prelude {
    pub use super::data::*;
    #[cfg(feature = "dev")]
    pub use super::dev::prelude::*;
    pub use super::world::prelude::*;
}

#[derive(Debug)]
pub struct ScreenPlugin {
    pub initial_screen: Screens,
}
impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        info!(?self);
        app.add_plugins(world::plugin).insert_state(CurrentScreen {
            screen: self.initial_screen,
            status: ScreenStatus::Loading,
        });

        #[cfg(feature = "dev")]
        app.add_plugins(dev::plugin);
    }
}

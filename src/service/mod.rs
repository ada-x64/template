// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------

use crate::prelude::*;

mod data;
#[cfg(feature = "dev")]
#[cfg(not(test))]
mod dev;
mod input;
mod player;
mod third_party;
mod ui;
mod worldgen;

pub mod prelude {
    pub use super::data::*;
    #[cfg(feature = "dev")]
    #[cfg(not(test))]
    pub use super::dev::prelude::*;
    pub use super::input::prelude::*;
    pub use super::player::prelude::*;
    pub use super::third_party::prelude::*;
    pub use super::worldgen::prelude::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins((
        third_party::plugin,
        input::plugin,
        ui::plugin,
        worldgen::plugin,
        player::plugin,
    ));
    #[cfg(feature = "dev")]
    #[cfg(not(test))]
    app.add_plugins(dev::plugin);
}

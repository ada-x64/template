// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
pub mod data;
pub mod input;
pub(crate) mod player;
pub mod ui;
pub mod worldgen;

use bevy::prelude::*;

use crate::third_party;

pub mod prelude {
    pub use super::data::*;
    pub use super::input::prelude::*;
    pub use super::player::prelude::*;
    pub use super::worldgen::prelude::*;
}

pub fn public_plugin(app: &mut App) {
    app.add_plugins((
        third_party::plugin,
        input::plugin,
        ui::plugin,
        worldgen::plugin,
    ));
    #[cfg(feature = "dev")]
    app.add_plugins(crate::dev::plugin);
}

pub fn private_plugin(app: &mut App) {
    app.add_plugins(player::plugin);
}

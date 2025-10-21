// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------

use crate::prelude::*;

mod camera;
mod cursor;
mod data;
mod events;
mod systems;

pub mod prelude {
    pub use super::camera::prelude::*;
    pub use super::cursor::prelude::*;
    pub use super::data::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins((events::plugin, systems::plugin))
        .add_plugins((cursor::plugin, camera::plugin))
        .add_input_context::<ICtxGlobal>();
}

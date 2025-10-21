// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
mod data;
mod events;
mod systems;

use crate::prelude::*;

pub mod prelude {
    pub use super::data::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins((systems::plugin, events::plugin))
        .add_input_context::<ICtxDefault>();
}

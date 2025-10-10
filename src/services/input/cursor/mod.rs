// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use crate::prelude::*;

mod data;
mod events;
mod systems;

pub mod prelude {
    pub use super::data::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins((events::plugin, systems::plugin))
        .add_input_context::<ICtxCaptureCursor>();
}

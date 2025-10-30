// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------

use crate::prelude::*;

mod bundle;
mod data;
mod events;

pub mod prelude {
    pub use super::bundle::flycam_bundle;
    pub use super::data::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(events::plugin)
        .add_input_context::<FlyCam>();
}

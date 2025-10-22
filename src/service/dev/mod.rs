// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use crate::prelude::*;

mod gizmos;
mod minibuffer;

pub mod prelude {
    pub use super::gizmos::prelude::*;
    pub use super::minibuffer::prelude::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins((minibuffer::plugin, gizmos::plugin));
}

// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use crate::prelude::*;

mod gizmos;
mod minibuffer;
mod test;
pub mod prelude {
    pub use super::gizmos::prelude::*;
    pub use super::minibuffer::prelude::*;
    pub use super::test::prelude::*;
}

pub fn plugin(app: &mut App) {
    #[cfg(not(test))]
    app.add_plugins((minibuffer::plugin, gizmos::plugin));
    #[cfg(test)]
    app.add_plugins(test::plugin);
}

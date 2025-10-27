// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use crate::prelude::*;

mod data;
mod world;

#[cfg(feature = "dev")]
mod dev;

#[cfg(test)]
mod test;

pub mod prelude {
    pub use super::data::*;
    #[cfg(feature = "dev")]
    pub use super::dev::prelude::*;
    #[cfg(test)]
    pub use super::test::prelude::*;
    pub use super::world::prelude::*;
}

pub fn plugin(app: &mut App) {
    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);
    #[cfg(test)]
    app.add_plugins(test::plugin);
    app.add_plugins(world::plugin);
}

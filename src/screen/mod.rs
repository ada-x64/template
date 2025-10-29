// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use crate::prelude::*;

mod data;
#[cfg(feature = "dev")]
mod dev;
mod splash;
mod world;

#[cfg(test)]
mod test;

pub mod prelude {
    pub use super::data::*;
    #[cfg(feature = "dev")]
    pub use super::dev::prelude::*;
    pub use super::splash::prelude::*;
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
    app.add_plugins(splash::plugin);
}

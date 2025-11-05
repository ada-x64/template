use crate::prelude::*;

mod data;
#[cfg(feature = "dev")]
mod dev;

mod splash;
#[cfg(test)]
mod test;
mod world;

pub mod prelude {
    pub use super::data::*;

    pub use super::splash::prelude::*;
    pub use super::world::prelude::*;

    #[allow(unused_imports)] // TEMP
    #[cfg(feature = "dev")]
    pub use super::dev::prelude::*;
    #[cfg(test)]
    pub use super::test::prelude::*;
}

pub fn plugin(app: &mut App) {
    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);
    #[cfg(test)]
    app.add_plugins(test::plugin);
    app.add_plugins(world::plugin);
    app.add_plugins(splash::plugin);
}

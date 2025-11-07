use crate::prelude::*;

mod runner;
mod screen;
mod tests;
mod util;

pub mod prelude {
    pub use super::runner::*;
    pub use super::screen::prelude::*;
    pub use super::util::*;
    pub use bevy_asset_loader::prelude::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(screen::plugin);
}

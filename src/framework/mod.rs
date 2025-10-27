use crate::prelude::*;

mod data;
mod events;
mod screen;
mod sysparams;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::data::*;
    pub use super::screen::prelude::*;
    pub use super::sysparams::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins((events::plugin, screen::plugin));
}

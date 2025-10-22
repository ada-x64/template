use crate::prelude::*;

mod data;
mod events;
mod screen;

pub mod prelude {
    pub use super::data::*;
    pub use super::screen::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(events::plugin);
}

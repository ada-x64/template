use crate::prelude::*;

mod data;
mod events;

pub mod prelude {
    pub use super::data::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(events::plugin);
}

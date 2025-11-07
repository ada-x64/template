use crate::prelude::*;

mod data;
mod empty;
mod named_entity;

pub mod prelude {
    pub use super::data::*;
    pub use super::empty::prelude::*;
    pub use super::named_entity::prelude::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins((named_entity::plugin, empty::plugin));
}

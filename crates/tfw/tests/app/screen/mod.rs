use crate::prelude::*;

mod blocking_scoped_system;
mod data;
mod empty;
mod lifecycle;
mod named_entity;
mod scoped_system;
pub mod prelude {
    pub use super::blocking_scoped_system::prelude::*;
    pub use super::data::*;
    pub use super::empty::prelude::*;
    pub use super::lifecycle::prelude::*;
    pub use super::named_entity::prelude::*;
    pub use super::scoped_system::prelude::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins((named_entity::plugin, empty::plugin));
    app.add_plugins(scoped_system::plugin);
    app.add_plugins(lifecycle::plugin);
    app.add_plugins(blocking_scoped_system::plugin);
}

use crate::prelude::*;

mod data;
mod events;

pub mod prelude {
    pub use super::data::*;
    // pub use my_submodule::prelude::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(events::plugin);
}

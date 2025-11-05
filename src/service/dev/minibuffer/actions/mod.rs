use crate::prelude::*;

mod in_world;

pub mod prelude {
    // pub use super::in_world::prelude::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(in_world::plugin);
}

use crate::prelude::*;

mod systems;

pub mod prelude {
    // use super::data::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(systems::plugin);
}

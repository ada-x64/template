use crate::prelude::*;

mod state;
mod systems;
pub mod prelude {
    pub use super::systems::TestScreen;
}

pub fn plugin(app: &mut App) {
    app.add_plugins((state::plugin, systems::plugin));
}

mod data;
mod events;
mod systems;

use crate::prelude::*;

pub mod prelude {
    pub use super::data::*;
    pub use super::systems::systems as player_systems;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(events::plugin)
        .add_input_context::<ICtxDefault>();
}

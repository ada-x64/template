// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use crate::prelude::*;

pub(crate) mod data;
pub(crate) mod world;

pub mod prelude {
    pub use super::data::*;
    pub use super::world::prelude::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(world::plugin)
        .add_systems(Update, log_state_transition)
        .init_state::<ScreenStates>();
}

fn log_state_transition(mut reader: EventReader<StateTransitionEvent<ScreenStates>>) {
    for ev in reader.read() {
        info!("{ev:?}");
    }
}

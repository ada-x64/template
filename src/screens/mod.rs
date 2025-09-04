// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use bevy::prelude::*;

pub(crate) mod loading;
pub(crate) mod main_menu;
pub(crate) mod splash;
pub(crate) mod world;

/// Loading is handled within the individual screens.
#[derive(States, PartialEq, Eq, Hash, Debug, Clone, Copy, Reflect)]
pub enum ScreenStates {
    Splash,
    MainMenu,
    InWorld,
}

pub fn plugin(app: &mut App) {
    app.add_plugins((world::plugin, main_menu::plugin, splash::plugin))
        .add_systems(Update, log_state_transition);
    // #[cfg(feature = "dev")]
    app.insert_state(ScreenStates::InWorld);
    // app.insert_state(ScreenStates::Splash);
}

fn log_state_transition(mut reader: EventReader<StateTransitionEvent<ScreenStates>>) {
    for ev in reader.read() {
        info!("{ev:?}");
    }
}

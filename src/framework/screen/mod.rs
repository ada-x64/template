use crate::{AppSettings, prelude::*};

mod data;
mod scope;
mod trait_impl;

pub mod prelude {
    pub use super::data::*;
    pub use super::scope::*;
    pub use super::trait_impl::*;
}

pub fn plugin(app: &mut App) {
    let settings = app.world().resource::<AppSettings>();
    app.insert_state(CurrentScreen {
        screen: settings.initial_screen,
        status: ScreenStatus::Loading,
    });
    app.init_resource::<NextScreen>();
}

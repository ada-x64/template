#![doc = include_str!("./doc.md")]

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
    app.insert_state::<CurrentScreen>(settings.initial_screen.into());
    app.insert_state::<CurrentScreenStatus>(ScreenStatus::Loading.into());
    app.init_resource::<NextScreen>();

    app.register_propagatable_type::<Persistent>();
    app.register_propagatable_type::<ScreenScoped>();
}

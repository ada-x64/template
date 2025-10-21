use crate::prelude::*;

mod actions;
mod inspectors;

pub mod prelude {
    // pub use super::actions::prelude::*;
    pub use super::inspectors::prelude::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins((actions::plugin, inspectors::plugin));
    app.add_acts(BasicActs::default());
}

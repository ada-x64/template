mod scoped_system;
use crate::prelude::*;

pub mod prelude {
    pub use super::scoped_system::prelude::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(scoped_system::plugin);
}

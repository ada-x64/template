use crate::prelude::*;

mod screen;
pub mod prelude {
    pub use super::screen::{LifecycleScreen, LifecycleSettings, LifecycleStatus};
}

pub fn plugin(app: &mut App) {
    app.add_plugins(screen::plugin);
}

use crate::prelude::*;

mod screen;
pub mod prelude {
    pub use super::screen::EmptyScreen;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(screen::plugin);
}

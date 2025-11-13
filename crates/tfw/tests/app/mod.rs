pub mod screen;
pub mod service;
pub mod util;

pub mod prelude {
    pub use super::screen::prelude::*;
    pub use super::service::prelude::*;
    pub use super::util::prelude::*;
    pub use bevy::prelude::*;
    pub use tfw::prelude::*;
}
use prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((screen::plugin, service::plugin));
}

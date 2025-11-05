use crate::prelude::*;

mod camera;
mod cursor;
mod data;
mod events;

pub mod prelude {
    pub use super::camera::prelude::*;
    pub use super::cursor::prelude::*;
    pub use super::data::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(events::plugin)
        .add_plugins((cursor::plugin, camera::plugin))
        .add_input_context::<ICtxGlobal>();
}

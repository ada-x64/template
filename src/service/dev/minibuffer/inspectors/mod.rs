use crate::prelude::*;

mod perf;
mod world;

pub mod prelude {
    pub use super::world::prelude::*;
    // pub use super::perf::prelude::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins((perf::plugin, world::plugin)).add_systems(
        Startup,
        |mut minibuffer: Minibuffer| {
            minibuffer.message("Dev mode enabled.");
            minibuffer.set_visible(true);
        },
    );
}

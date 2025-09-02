use bevy::prelude::*;
use bevy_minibuffer::prelude::*;

pub(crate) mod perf;
pub(crate) mod world;

pub fn plugin(app: &mut App) {
    app.add_plugins((perf::plugin, world::plugin)).add_systems(
        Startup,
        |mut minibuffer: Minibuffer| {
            minibuffer.message("Dev mode enabled.");
            minibuffer.set_visible(true);
        },
    );
}

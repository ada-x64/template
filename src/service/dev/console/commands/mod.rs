use crate::prelude::*;

mod set_cam;

pub fn plugin(app: &mut App) {
    app.add_plugins(set_cam::plugin);
}

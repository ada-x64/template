use crate::prelude::*;

mod gizmos;
pub mod prelude {
    pub use super::gizmos::prelude::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(gizmos::plugin);
}

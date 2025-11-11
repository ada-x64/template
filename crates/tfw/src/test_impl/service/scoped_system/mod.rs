use crate::prelude::*;

mod data;
mod systems;

pub mod prelude {
    pub use super::data::*;
    pub use super::systems::systems as scoped_service_systems;
}

pub fn plugin(app: &mut App) {
    app.init_resource::<ScopedSystemValue>();
}

use crate::prelude::*;

mod gizmos;
 mod camera_test; pub mod prelude { pub use super::camera_test::prelude::*; 
    pub use super::gizmos::prelude::*;
}

pub fn plugin(app: &mut App) {
    #[cfg(not(test))]
    app.add_plugins(gizmos::plugin);
 app.add_plugins(camera_test::plugin); }

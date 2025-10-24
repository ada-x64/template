use crate::prelude::*;

mod test;

pub mod prelude {
    pub use super::test::prelude::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(test::plugin);
}

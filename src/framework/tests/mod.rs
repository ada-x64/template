use crate::{AppPlugin, prelude::*};

#[test]
fn screen_transitions() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AppPlugin::default()));
}

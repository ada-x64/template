use crate::{AppPlugin, AppSettings, prelude::*};

#[test]
fn screen_transitions() {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        AppPlugin {
            settings: AppSettings {
                initial_screen: Screens::Test,
            },
        },
    ));
    app.update();
    let mut q = app.world_mut().query::<&Name>();
    assert!(q.iter(app.world()).any(|name| (**name).eq("Hello")));
    app.world_mut().trigger(SwitchScreen(Screens::InWorld));
}

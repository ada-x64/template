mod entity_scoping;
mod system_scoping;

use crate::prelude::*;
use tfw::{TfwPlugin, TfwSettings};

// the rest of the tests use non-blocking asset loading to circumvent the delay.
// but this prevents the use of on_ready. and we need to test that loading blocking works.
#[test]
fn lifecycle() {
    let mut app = App::new();
    app.add_plugins((
        TestRunnerPlugin::default(),
        AppPlugin,
        TfwPlugin {
            settings: TfwSettings {
                initial_screen: LifecycleScreen::name(),
            },
        },
    ));
    app.add_systems(
        OnEnter(ScreenState::<LifecycleScreen>::Loading),
        |mut r: ResMut<LifecycleStatus>| {
            info!("loading");
            r.loading = true;
        },
    );
    app.add_systems(
        OnEnter(ScreenState::<LifecycleScreen>::Ready),
        |mut r: ResMut<LifecycleStatus>, mut commands: Commands| {
            info!("ready");
            r.ready = true;
            commands.trigger(SwitchToScreen::<EmptyScreen>::default());
        },
    );
    app.add_systems(
        OnEnter(ScreenState::<LifecycleScreen>::Unloaded),
        |mut r: ResMut<LifecycleStatus>| {
            info!("unloaded");
            r.unloaded = true;
        },
    );
    app.add_systems(
        OnEnter(ScreenState::<EmptyScreen>::Ready),
        |r: Res<LifecycleStatus>, mut commands: Commands| {
            let ok = r.loading && r.ready && r.unloaded && r.in_init && r.in_unload;
            if ok {
                commands.write_message(AppExit::Success);
            } else {
                error!("Did not reach all expected points.");
                error!(?r);
                commands.write_message(AppExit::error());
            }
        },
    );
    assert!(app.run().is_success());
}

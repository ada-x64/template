mod runner;

use crate::{AppPlugin, AppSettings, framework::tests::runner::Runner, prelude::*};

#[test]
fn screen_transitions() {
    Runner::new(|app| {
        info!("Testing screen transitions.");
        app.add_plugins((AppPlugin {
            settings: AppSettings {
                initial_screen: Screens::Test,
            },
        },));
        info!("Plugins registered.");
        {
            let mut settings = app.world_mut().resource_mut::<TestScreenSettings>();
            settings.entity_name = "1".into();
            info!("Settings updated.");
        }
        {
            app.world_mut().trigger(SwitchToScreen(Screens::Test));
            app.update();
            info!("World updated.");
            // dump world state
            let mut q = app.world_mut().query_filtered::<Entity, Without<ChildOf>>();
            let state = q.iter(app.world()).map(|e| print!("{e:#?}"));
            info!(?state);
            let mut q = app.world_mut().query::<&Name>();
            assert!(q.iter(app.world()).any(|name| (**name).eq("1")));
            info!("Found first entity.");
        }
        {
            let mut settings = app.world_mut().resource_mut::<TestScreenSettings>();
            settings.entity_name = "2".into();
        }
        {
            app.world_mut().trigger(SwitchToScreen(Screens::Test));
            app.update();
            // dump world state
            let mut q = app.world_mut().query_filtered::<Entity, Without<ChildOf>>();
            let state = q.iter(app.world()).map(|e| print!("{e:#?}"));
            info!(?state);
            let mut q = app.world_mut().query::<&Name>();
            assert!(!q.iter(app.world()).any(|name| (**name).eq("1")));
            assert!(q.iter(app.world()).any(|name| (**name).eq("2")));
        };
        AppExit::Success
    })
    .run();
}

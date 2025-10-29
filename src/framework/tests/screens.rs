use crate::{AppPlugin, AppSettings, framework::tests::*, prelude::*};

#[test]
fn screen_transitions() {
    Runner::new(|app| {
        app.add_plugins((AppPlugin {
            settings: AppSettings {
                initial_screen: Screens::Empty,
            },
        },));
        // Check that the screen runs init and populates the named entity.
        {
            {
                let mut settings = app.world_mut().resource_mut::<NamedEntityScreenSettings>();
                settings.entity_name = "1".into();
                info!("Settings updated.");
            }
            switch_screen(app, Screens::NamedEntity);
            let mut q = app.world_mut().query::<&Name>();
            assert!(q.iter(app.world()).any(|name| (**name).eq("1")));
            info!("Found first entity.");
        }
        // Check that screen transitions clear non-persistent entities.
        {
            switch_screen(app, Screens::Empty);
            let mut q = app.world_mut().query::<&Name>();
            assert!(!q.iter(app.world()).any(|name| (**name).eq("1")));
            info!("Cleared entity.");
        }
        // Check that updating settings effects next load.
        {
            {
                let mut settings = app.world_mut().resource_mut::<NamedEntityScreenSettings>();
                settings.entity_name = "2".into();
                info!("Settings updated.");
            }
            switch_screen(app, Screens::NamedEntity);
            let mut q = app.world_mut().query::<&Name>();
            assert!(q.iter(app.world()).any(|name| (**name).eq("2")));
            info!("Found second entity.");
        };
        AppExit::Success
    })
    .with_timeout(5.)
    .run();
}

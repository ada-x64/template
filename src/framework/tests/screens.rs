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
            let mut settings = app.world_mut().resource_mut::<NamedEntityScreenSettings>();
            settings.entity_name = "1".into();
        }
        switch_screen(app, Screens::NamedEntity);
        assert!(find_entity(app, "1"));

        // Check that screen transitions clear non-persistent entities.
        switch_screen(app, Screens::Empty);
        find_entity(app, "1");
        assert!(!find_entity(app, "1"));

        // Check that updating settings effects next load.
        {
            let mut settings = app.world_mut().resource_mut::<NamedEntityScreenSettings>();
            settings.entity_name = "2".into();
        }
        switch_screen(app, Screens::NamedEntity);
        assert!(find_entity(app, "2"));
        AppExit::Success
    })
    .run();
}

#[test]
fn persistent_entities() {
    Runner::new(|app| {
        app.add_plugins((AppPlugin {
            settings: AppSettings {
                initial_screen: Screens::NamedEntity,
            },
        },));
        {
            let mut settings = app.world_mut().resource_mut::<NamedEntityScreenSettings>();
            settings.entity_name = "1".into();
            app.world_mut().spawn((
                Name::new("Persistent"),
                Propogate(Persistent),
                children![(
                    Name::new("Child"),
                    children![(
                        Name::new("Grandchild"),
                        BlockPropogation::<Persistent>::default(),
                        children![Name::new("Great Grandchild")]
                    )]
                )],
            ));
        }
        app.update();
        log_hierarchy(app);
        assert!(find_entity(app, "Persistent"));
        assert!(find_entity_filtered::<With<Persistent>>(app, "Child"));
        assert!(find_entity_filtered::<Without<Persistent>>(
            app,
            "Grandchild"
        ));
        assert!(find_entity_filtered::<(Without<Persistent>,)>(
            app,
            "Great Grandchild"
        ));

        switch_screen(app, Screens::Empty);
        assert!(find_entity(app, "Persistent"));
        assert!(find_entity(app, "Child"));
        assert!(!find_entity(app, "Grandchild"));
        assert!(!find_entity(app, "Great Grandchild"));
        assert!(!find_entity(app, "1"));
        AppExit::Success
    })
    .run();
}

// Child observers should be removed, but top-level observers should remain.
#[test]
fn observer_cleanup() {
    Runner::new(|app| {
        app.add_plugins((AppPlugin {
            settings: AppSettings {
                initial_screen: Screens::Empty,
            },
        },))
            .init_resource::<Count>();

        log_hierarchy(app);
        {
            app.world_mut().spawn((
                Name::new("Parent"),
                children![(
                    Name::new("Child"),
                    Observer::new(|trigger: Trigger<SwitchToScreen>| {
                        info!("Observer ({:?})", *trigger)
                    })
                )],
            ));
        }
        assert!(find_entity(app, "Parent"));
        assert!(find_entity(app, "Child"));

        switch_screen(app, Screens::NamedEntity);
        assert!(!find_entity(app, "Parent"));
        assert!(!find_entity(app, "Child"));
        AppExit::Success
    })
    .run();
}

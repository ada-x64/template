use bevy::ecs::entity_disabling::Internal;

use crate::{AppPlugin, AppSettings, framework::tests::*, prelude::*};

#[test]
fn screen_transitions() {
    Runner::new(|app| {
        app.add_plugins((AppPlugin {
            settings: AppSettings {
                initial_screen: Screens::Empty,
                ..Default::default()
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
                ..Default::default()
            },
        },));
        {
            let mut settings = app.world_mut().resource_mut::<NamedEntityScreenSettings>();
            settings.entity_name = "1".into();
            app.world_mut().spawn((
                Name::new("Persistent"),
                Propagate(Persistent),
                children![(
                    Name::new("Child"),
                    children![(
                        Name::new("Grandchild"),
                        BlockPropagation::<Persistent>::default(),
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

#[derive(Component)]
struct Empty;

/// Child observers should be removed, but top-level observers should remain.
/// NOTE: Child observers _probably_ shouldn't exist. This functionality has been
/// replaced with [EntityEvent]
#[test]
fn observer_cleanup() {
    Runner::new(|app| {
        app.add_plugins((AppPlugin {
            settings: AppSettings {
                initial_screen: Screens::Empty,
                ..Default::default()
            },
        },));

        {
            app.world_mut().spawn((
                Name::new("Parent"),
                children![(
                    Name::new("Child"),
                    Observer::new(|trigger: On<SwitchToScreen>| {
                        info!("Observer ({:?})", *trigger)
                    }),
                    Empty
                )],
            ));
        }
        log_hierarchy(app);
        assert!(find_entity(app, "Parent"));
        assert!(find_entity_filtered::<Allow<Internal>>(app, "Child"));

        switch_screen(app, Screens::NamedEntity);
        assert!(!find_entity(app, "Parent"));
        assert!(!find_entity_filtered::<Allow<Internal>>(app, "Child"));
        AppExit::Success
    })
    .run();
}

mod app;
pub use app::prelude;
use prelude::*;

use bevy::ecs::entity_disabling::Internal;

use tfw::{TfwPlugin, TfwSettings};

#[test]
fn screen_transitions() {
    Runner::new(|app| {
        debug!("1");
        app.add_plugins((
            app::plugin,
            TfwPlugin {
                settings: TfwSettings {
                    initial_screen: Screens::Empty.into(),
                },
            },
        ));
        app.init_resource::<Step>();
        app.add_systems(
            Update,
            |mut step: ResMut<Step>,
             mut settings: ResMut<NamedEntityScreenSettings>,
             mut commands: Commands,
             empty_screen_state: Res<State<ScreenLoadingState<EmptyScreen>>>,
             named_entity_state: Res<State<ScreenLoadingState<NamedEntityScreen>>>,
             q: Query<(Entity, &Name)>| {
                match **step {
                    // set up named entity screen
                    0 => {
                        debug!("step 0");
                        if !matches!(**empty_screen_state, ScreenLoadingState::Ready) {
                            return;
                        }
                        settings.entity_name = "1".into();
                        commands.trigger(SwitchToScreen(Screens::NamedEntity.into()));
                    }
                    // assert that the named entity exists after load
                    1 => {
                        debug!("step 1");
                        if !matches!(**named_entity_state, ScreenLoadingState::Ready) {
                            return;
                        }
                        let found = q.iter().any(|(_, ename)| (**ename).eq("1"));
                        assert!(found);
                        commands.trigger(SwitchToScreen(Screens::Empty.into()));
                    }
                    // Check that screen transitions clear non-persistent entities.
                    2 => {
                        debug!("step 2");
                        if !matches!(**empty_screen_state, ScreenLoadingState::Ready) {
                            return;
                        }
                        let found = q.iter().any(|(_, ename)| (**ename).eq("1"));
                        assert!(!found);
                        settings.entity_name = "2".into();
                    }
                    // Check that updating settings affects next load.
                    3 => {
                        debug!("step 3");
                        if !matches!(**named_entity_state, ScreenLoadingState::Ready) {
                            return;
                        }
                        let found = q.iter().any(|(_, ename)| (**ename).eq("2"));
                        assert!(found);
                        commands.write_message(AppExit::Success);
                    }
                    _ => {
                        unreachable!();
                    }
                }
                **step += 1;
            },
        );
        app.run().is_success()
    })
    .run();
}

#[test]
fn persistent_entities() {
    Runner::new(|app| {
        app.add_plugins((
            app::plugin,
            TfwPlugin {
                settings: TfwSettings {
                    initial_screen: Screens::NamedEntity.into(),
                },
            },
        ));
        {
            let mut settings = app.world_mut().resource_mut::<NamedEntityScreenSettings>();
            settings.entity_name = "1".into();
            app.world_mut().spawn((
                Name::new("Persistent"),
                bevy::app::Propagate(Persistent),
                children![(
                    Name::new("Child"),
                    children![(
                        Name::new("Grandchild"),
                        bevy::app::PropagateStop::<Persistent>::default(),
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
        app.update();
        assert!(find_entity(app, "Persistent"));
        assert!(find_entity(app, "Child"));
        assert!(!find_entity(app, "Grandchild"));
        assert!(!find_entity(app, "Great Grandchild"));
        assert!(!find_entity(app, "1"));
        true
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
        app.add_plugins((
            app::plugin,
            TfwPlugin {
                settings: TfwSettings {
                    initial_screen: Screens::Empty.into(),
                },
            },
        ));

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
        app.update();
        assert!(!find_entity(app, "Parent"));
        assert!(!find_entity_filtered::<Allow<Internal>>(app, "Child"));
        true
    })
    .run();
}

#[test]
fn scoped_systems() {
    Runner::new(|app| {
        app.add_plugins((
            app::plugin,
            TfwPlugin {
                settings: TfwSettings {
                    initial_screen: Screens::ScopedSystem.into(),
                },
            },
        ));
        app.init_resource::<Step>();
        app.add_systems(
            Update,
            |mut step: ResMut<Step>,
             screen_state: Res<State<ScreenLoadingState<ScopedSystemScreen>>>,
             empty_screen_state: Res<State<ScreenLoadingState<ScopedSystemScreen>>>,
             mut commands: Commands,
             value: Res<ScopedSystemValue>| {
                debug!("here");
                match **step {
                    0..=2 => {
                        if !screen_state.is_ready() {
                            return;
                        }
                        debug!(?step, ?value);
                        assert_eq!(**value, **step + 1);
                        **step += 1;
                        commands.write_message(AppExit::error()); // to test
                    }
                    3 => {
                        commands.trigger(SwitchToScreen(Screens::Empty.into()));
                        **step += 1;
                    }
                    // assert switching resets the value
                    4 => {
                        if !empty_screen_state.is_ready() {
                            return;
                        }
                        debug!(?step, ?value);
                        assert_eq!(**value, 3);
                        commands.trigger(SwitchToScreen(Screens::ScopedSystem.into()));
                        **step += 1;
                    }
                    // assert switching resets the value, i.e. runs on_ready
                    5 => {
                        if !screen_state.is_ready() {
                            return;
                        }
                        debug!(?step, ?value);
                        assert_eq!(**value, 1);
                        commands.write_message(AppExit::Success);
                    }
                    _ => unreachable!(),
                }
            },
        );
        debug!("2");
        app.run().is_success()
    })
    .run();
}

mod app;
pub use app::prelude;
use prelude::*;

use bevy::ecs::entity_disabling::Internal;

use tfw::{TfwPlugin, TfwSettings};

#[test]
fn screen_transitions() {
    let mut app = App::new();
    app.add_plugins((
        TestRunnerPlugin { timeout: 1. },
        app::plugin,
        TfwPlugin {
            settings: TfwSettings {
                initial_screen: Screens::Empty.into(),
            },
        },
    ));
    app.init_resource::<Step>();
    let log = |world: &mut World, mut count: Local<u32>, mut last_step: Local<u32>| {
        log_hierarchy(world);
        let step = world.resource::<Step>();
        if **step != *last_step {
            debug!(?step, ?count);
            *last_step += 1;
        }
        *count += 1;
    };
    let update = |mut step: ResMut<Step>,
                  mut settings: ResMut<NamedEntityScreenSettings>,
                  mut commands: Commands,
                  q: Query<(Entity, &Name)>| {
        match **step {
            // set up named entity screen
            0 => {
                settings.entity_name = "1".into();
                commands.trigger(SwitchToScreen(Screens::NamedEntity.into()));
            }
            // assert that the named entity exists after load
            1 => {
                let found = q.iter().any(|(_, ename)| {
                    debug!(?ename);
                    (**ename).eq("1")
                });
                if !found {
                    error!("Could not find entity with name '1'");
                    commands.write_message(AppExit::error());
                    return;
                }
                commands.trigger(SwitchToScreen(Screens::Empty.into()));
            }
            // Check that screen transitions clear non-persistent entities.
            2 => {
                let found = q.iter().any(|(_, ename)| (**ename).eq("1"));
                if found {
                    error!("Could not find entity with name '1'");
                    commands.write_message(AppExit::error());
                    return;
                }
                settings.entity_name = "2".into();
                commands.trigger(SwitchToScreen(Screens::NamedEntity.into()));
            }
            // Check that updating settings affects next load.
            3 => {
                let found = q.iter().any(|(_, ename)| (**ename).eq("2"));
                if !found {
                    error!("Could not find entity with name '2'");
                    commands.write_message(AppExit::error());
                    return;
                }
                commands.write_message(AppExit::Success);
            }
            _ => {
                unreachable!();
            }
        }
        **step += 1;
    };
    app.add_systems(PostUpdate, (log, update).chain());
    assert!(app.run().is_success());
}

#[test]
fn persistent_entities() {
    let mut app = App::new();
    app.add_plugins((
        TestRunnerPlugin::default(),
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
    log_hierarchy(app.world_mut());
    assert!(find_entity(&mut app, "Persistent"));
    assert!(find_entity_filtered::<With<Persistent>>(&mut app, "Child"));
    assert!(find_entity_filtered::<Without<Persistent>>(
        &mut app,
        "Grandchild"
    ));
    assert!(find_entity_filtered::<(Without<Persistent>,)>(
        &mut app,
        "Great Grandchild"
    ));

    switch_screen(&mut app, Screens::Empty);
    app.update();
    assert!(find_entity(&mut app, "Persistent"));
    assert!(find_entity(&mut app, "Child"));
    assert!(!find_entity(&mut app, "Grandchild"));
    assert!(!find_entity(&mut app, "Great Grandchild"));
    assert!(!find_entity(&mut app, "1"));
}

#[derive(Component)]
struct Empty;

/// Child observers should be removed, but top-level observers should remain.
/// NOTE: Child observers _probably_ shouldn't exist. This functionality has been
/// replaced with [EntityEvent]
#[test]
fn observer_cleanup() {
    let mut app = App::new();
    app.add_plugins((
        TestRunnerPlugin::default(),
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
                Observer::new(|trigger: On<SwitchToScreen>| { info!("Observer ({:?})", *trigger) }),
                Empty
            )],
        ));
    }
    log_hierarchy(app.world_mut());
    assert!(find_entity(&mut app, "Parent"));
    assert!(find_entity_filtered::<Allow<Internal>>(&mut app, "Child"));

    switch_screen(&mut app, Screens::NamedEntity);
    app.update();
    assert!(!find_entity(&mut app, "Parent"));
    assert!(!find_entity_filtered::<Allow<Internal>>(&mut app, "Child"));
}

// the rest of the tests use non-blocking asset loading to circumvent the delay.
// but this prevents the use of on_ready. and we need to test that loading blocking works.
#[test]
fn blocking_load() {
    todo!();
}

#[test]
fn scoped_systems() {
    let mut app = App::new();
    app.add_plugins((
        TestRunnerPlugin { timeout: 1. },
        app::plugin,
        TfwPlugin {
            settings: TfwSettings {
                initial_screen: Screens::ScopedSystem.into(),
            },
        },
    ));
    app.init_resource::<Step>();
    app.add_systems(
        PostUpdate,
        |mut step: ResMut<Step>, mut commands: Commands, value: Res<ScopedSystemValue>| {
            debug!("PostUpdate");
            match **step {
                0..=2 => {
                    debug_once!(?step, ?value);
                    if **value != **step + 1 {
                        error!("step value did not match");
                        commands.write_message(AppExit::error());
                        return;
                    }
                    **step += 1;
                }
                3 => {
                    debug_once!(?step, ?value);
                    commands.trigger(SwitchToScreen(Screens::Empty.into()));
                    **step += 1;
                }
                // assert value does not update when in different screen
                4 => {
                    debug_once!(?step, ?value);
                    if **value != 4 {
                        error!("step value did not match");
                        commands.write_message(AppExit::error());
                        return;
                    }
                    commands.trigger(SwitchToScreen(Screens::ScopedSystem.into()));
                    **step += 1;
                }
                // assert switching continues to increment
                5 => {
                    debug_once!(?step, ?value);
                    if **value != 5 {
                        error!("step value did not match");
                        commands.write_message(AppExit::error());
                        return;
                    }
                    commands.write_message(AppExit::Success);
                }
                _ => unreachable!(),
            }
        },
    );
    assert!(app.run().is_success());
}

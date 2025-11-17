mod scoping;

use crate::prelude::*;
use bevy::ecs::entity_disabling::Internal;
use tfw::{TfwPlugin, TfwSettings};

// #[test]
// fn screen_transitions() {
//     let mut app = App::new();
//     app.add_plugins((
//         TestRunnerPlugin { timeout: 1. },
//         AppPlugin,
//         TfwPlugin {
//             settings: TfwSettings {
//                 initial_screen: EmptyScreen::name(),
//             },
//         },
//     ));
//     app.init_resource::<Step>();
//     let log = |world: &mut World, mut count: Local<u32>, mut last_step: Local<u32>| {
//         log_hierarchy(world);
//         let step = world.resource::<Step>();
//         if **step != *last_step {
//             info!(?step, ?count);
//             *last_step += 1;
//         }
//         *count += 1;
//     };
//     let update = |mut step: ResMut<Step>,
//                   mut settings: ResMut<NamedEntityScreenSettings>,
//                   mut commands: Commands,
//                   q: Query<(Entity, &Name)>| {
//         match **step {
//             // set up named entity screen
//             0 => {
//                 settings.entity_name = "1".into();
//                 commands.trigger(SwitchToScreen::<NamedEntityScreen>::default());
//             }
//             // assert that the named entity exists after load
//             1 => {
//                 let found = q.iter().any(|(_, ename)| {
//                     info!(?ename);
//                     (**ename).eq("1")
//                 });
//                 if !found {
//                     error!("Could not find entity with name '1'");
//                     commands.write_message(AppExit::error());
//                     return;
//                 }
//                 commands.trigger(SwitchToScreen::<EmptyScreen>::default());
//             }
//             // Check that screen transitions clear non-persistent entities.
//             2 => {
//                 let found = q.iter().any(|(_, ename)| (**ename).eq("1"));
//                 if found {
//                     error!("Could not find entity with name '1'");
//                     commands.write_message(AppExit::error());
//                     return;
//                 }
//                 settings.entity_name = "2".into();
//                 commands.trigger(SwitchToScreen::<NamedEntityScreen>::default());
//             }
//             // Check that updating settings affects next load.
//             3 => {
//                 let found = q.iter().any(|(_, ename)| (**ename).eq("2"));
//                 if !found {
//                     error!("Could not find entity with name '2'");
//                     commands.write_message(AppExit::error());
//                     return;
//                 }
//                 commands.write_message(AppExit::Success);
//             }
//             _ => {
//                 unreachable!();
//             }
//         }
//         **step += 1;
//     };
//     app.add_systems(PostUpdate, (log, update).chain());
//     assert!(app.run().is_success());
// }

#[test]
fn persistent_entities() {
    let mut app = App::new();
    app.add_plugins((
        TestRunnerPlugin::default(),
        AppPlugin,
        TfwPlugin {
            settings: TfwSettings {
                initial_screen: NamedEntityScreen::name(),
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

    app.world_mut()
        .trigger(SwitchToScreen::<EmptyScreen>::default());
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
        AppPlugin,
        TfwPlugin {
            settings: TfwSettings {
                initial_screen: EmptyScreen::name(),
            },
        },
    ));

    {
        app.world_mut().spawn((
            Name::new("Parent"),
            children![(
                Name::new("Child"),
                Observer::new(|trigger: On<SwitchToScreen<NamedEntityScreen>>| {
                    info!("Observer ({:?})", *trigger)
                }),
                Empty
            )],
        ));
    }
    log_hierarchy(app.world_mut());
    assert!(find_entity(&mut app, "Parent"));
    assert!(find_entity_filtered::<Allow<Internal>>(&mut app, "Child"));

    app.world_mut()
        .trigger(SwitchToScreen::<NamedEntityScreen>::default());
    app.update();
    assert!(!find_entity(&mut app, "Parent"));
    assert!(!find_entity_filtered::<Allow<Internal>>(&mut app, "Child"));
}

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

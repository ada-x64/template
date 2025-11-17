use bevy::ecs::entity_disabling::Internal;
use tfw::{TfwPlugin, TfwSettings};

use crate::prelude::*;

// NOTE: All of these use nonblocking loading strategies, so they do not have to
// check for ScreenStatus::Ready

#[test]
fn screen_transitions() {
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

    app.add_step(
        0,
        |mut step: ResMut<NextState<Step>>,
         mut settings: ResMut<NamedEntityScreenSettings>,
         mut commands: Commands| {
            settings.entity_name = "1".into();
            commands.trigger(SwitchToScreen::<NamedEntityScreen>::default());
            step.set(Step(1));
        },
    )
    .add_step(
        1,
        |mut step: ResMut<NextState<Step>>, mut commands: Commands, q: Query<(Entity, &Name)>| {
            let found = q.iter().any(|(_, ename)| {
                info!(?ename);
                (**ename).eq("1")
            });
            if !found {
                error!("Could not find entity with name '1'");
                commands.write_message(AppExit::error());
                return;
            }
            commands.trigger(SwitchToScreen::<EmptyScreen>::default());
            step.set(Step(2));
        },
    )
    .add_step(
        2,
        |mut step: ResMut<NextState<Step>>,
         mut settings: ResMut<NamedEntityScreenSettings>,
         mut commands: Commands,
         q: Query<(Entity, &Name)>| {
            let found = q.iter().any(|(_, ename)| (**ename).eq("1"));
            if found {
                error!("Could not find entity with name '1'");
                commands.write_message(AppExit::error());
                return;
            }
            settings.entity_name = "2".into();
            commands.trigger(SwitchToScreen::<NamedEntityScreen>::default());
            step.set(Step(3));
        },
    )
    .add_step(3, |mut commands: Commands, q: Query<(Entity, &Name)>| {
        let found = q.iter().any(|(_, ename)| (**ename).eq("2"));
        if !found {
            error!("Could not find entity with name '2'");
            commands.write_message(AppExit::error());
            return;
        }
        commands.write_message(AppExit::Success);
    });

    assert!(app.run().is_success());
}

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
    app.world_mut().commands().log_hierarchy();
    assert!(app.find_entity("Persistent"));
    assert!(app.find_entity_filtered::<With<Persistent>>("Child"));
    assert!(app.find_entity_filtered::<Without<Persistent>>("Grandchild"));
    assert!(app.find_entity_filtered::<(Without<Persistent>,)>("Great Grandchild"));

    app.world_mut()
        .trigger(SwitchToScreen::<EmptyScreen>::default());
    app.update();
    assert!(app.find_entity("Persistent"));
    assert!(app.find_entity("Child"));
    assert!(!app.find_entity("Grandchild"));
    assert!(!app.find_entity("Great Grandchild"));
    assert!(!app.find_entity("1"));
}

#[derive(Component)]
struct Empty;

/// Child observers should be removed, but top-level observers should remain.
/// NOTE: Child observers _probably_ shouldn't exist. This functionality has been
/// replaced with [EntityEvent]. But, this checks the scoping query functions as intended.
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
    app.world_mut().commands().log_hierarchy();
    assert!(app.find_entity("Parent"));
    assert!(app.find_entity_filtered::<Allow<Internal>>("Child"));

    app.world_mut()
        .trigger(SwitchToScreen::<NamedEntityScreen>::default());
    app.update();
    assert!(!app.find_entity("Parent"));
    assert!(!app.find_entity_filtered::<Allow<Internal>>("Child"));
}

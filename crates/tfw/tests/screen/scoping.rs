use tfw::{TfwPlugin, TfwSettings};

use crate::prelude::*;

#[derive(Resource, Deref, DerefMut, Default)]
struct SavedValue(u32);

#[test]
fn nonblocking() {
    let mut app = App::new();
    app.add_plugins((
        TestRunnerPlugin::default(),
        AppPlugin,
        TfwPlugin {
            settings: TfwSettings {
                initial_screen: ScopedSystemScreen::name(),
            },
        },
    ));
    app.init_state::<Step>();
    app.init_resource::<SavedValue>();
    app.add_systems(
        PostUpdate,
        (|mut step: ResMut<NextState<Step>>,
          mut commands: Commands,
          mut saved_value: ResMut<SavedValue>,
          value: Res<ScopedSystemValue>,
          screen_state: Res<State<ScreenState<ScopedSystemScreen>>>| {
            if !screen_state.is_ready() {
                return;
            }
            info_once!(?value);
            step.set(Step(1));
            // this should _immediately_ freeze the screen-scoped systems
            commands.trigger(SwitchToScreen::<EmptyScreen>::default());
            **saved_value = **value;
        })
        .run_if(in_state(Step(0))),
    );
    // assert that systems have frozen, i.e. value does not increment while unloading
    // or while in a different screen
    app.add_systems(
        PostUpdate,
        (|mut step: ResMut<NextState<Step>>,
          mut commands: Commands,
          value: Res<ScopedSystemValue>,
          saved_value: Res<SavedValue>,
          screen_state: Res<State<ScreenState<EmptyScreen>>>| {
            if !screen_state.is_ready() {
                return;
            }
            info_once!(?value);
            if **value != **saved_value {
                error!("Failed to match value!");
                commands.write_message(AppExit::error());
                return;
            }
            commands.trigger(SwitchToScreen::<ScopedSystemScreen>::default());
            step.set(Step(2));
        })
        .run_if(in_state(Step(1))),
    );
    // assert that value increments once returned to the screen
    app.add_systems(
        PostUpdate,
        (|mut commands: Commands,
          value: Res<ScopedSystemValue>,
          saved_value: ResMut<SavedValue>,
          screen_state: Res<State<ScreenState<ScopedSystemScreen>>>| {
            if !screen_state.is_ready() {
                return;
            }
            info_once!(?value);
            if **value == **saved_value {
                error!("Failed to increment value!");
                commands.write_message(AppExit::error());
            } else {
                commands.write_message(AppExit::Success);
            }
        })
        .run_if(in_state(Step(2))),
    );
    assert!(app.run().is_success());
}

#[test]
fn blocking() {
    type Screen = BlockingScopedSystemScreen;
    type Settings = BlockingScopedSystemSettings;
    type Value = BlockingScopedSystemValue;

    let mut app = App::new();
    app.add_plugins((
        TestRunnerPlugin::default(),
        AppPlugin,
        TfwPlugin {
            settings: TfwSettings {
                initial_screen: Screen::name(),
            },
        },
    ));
    app.init_state::<Step>();
    app.insert_resource(Settings {
        initial_value: 100,
        unload_value: 200,
    });
    app.add_systems(
        PostUpdate,
        (|mut step: ResMut<NextState<Step>>,
          mut commands: Commands,
          settings: Res<Settings>,
          value: Res<Value>,
          screen_state: Res<State<ScreenState<Screen>>>| {
            if !screen_state.is_ready() {
                if *value != Value::default() {
                    error!("Got spurious value change!");
                    info!("Step = 1, Value = {}", **value);
                    commands.write_message(AppExit::error());
                }
            } else {
                // +1 because it will have updated by now
                if **value != settings.initial_value + 1 {
                    error!("Did not get value change on ready!");
                    info!("Step = 1, Value = {}", **value);
                    commands.write_message(AppExit::error());
                }
                step.set(Step(1));
            }
        })
        .run_if(in_state(Step(0))),
    );
    // let it increment for a bit
    app.add_systems(
        PostUpdate,
        (|mut step: ResMut<NextState<Step>>,
          mut commands: Commands,
          settings: Res<Settings>,
          value: Res<Value>| {
            if **value < settings.initial_value + 5 {
                return;
            }
            // this immediately runs unload, should be frozen at unload_value
            commands.trigger(SwitchToScreen::<EmptyScreen>::default());
            step.set(Step(2));
        })
        .run_if(in_state(Step(1))),
    );

    // assert that systems have frozen, i.e. value does not increment while unloading
    // or while in a different screen
    app.add_systems(
        PostUpdate,
        (|mut step: ResMut<NextState<Step>>,
          mut commands: Commands,
          value: Res<Value>,
          settings: Res<Settings>,
          screen_state: Res<State<ScreenState<EmptyScreen>>>| {
            // assert value has been frozen
            if **value != settings.unload_value {
                error!("Value does not match unload_value");
                info!("Step = 2, Value = {}", **value);
                commands.write_message(AppExit::error());
            }
            if !screen_state.is_ready() {
                return;
            }
            commands.trigger(SwitchToScreen::<Screen>::default());
            step.set(Step(3));
        })
        .run_if(in_state(Step(2))),
    );
    // assert that value resets on init
    app.add_systems(
        PostUpdate,
        (|mut commands: Commands,
          value: Res<Value>,
          settings: Res<Settings>,
          screen_state: Res<State<ScreenState<Screen>>>| {
            if !screen_state.is_ready() {
                if **value != settings.unload_value {
                    error!("Got spurious value change!");
                    info!("Step = 3, Value = {}", **value);
                    commands.write_message(AppExit::error());
                }
            } else if **value != settings.initial_value + 1 {
                error!("Did not get value change on ready!");
                info!("Step = 3, Value = {}", **value);
                commands.write_message(AppExit::error());
            } else {
                commands.write_message(AppExit::Success);
            }
        })
        .run_if(in_state(Step(3))),
    );
    assert!(app.run().is_success());
}

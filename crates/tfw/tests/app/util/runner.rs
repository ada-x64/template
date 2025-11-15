use bevy::{
    app::ScheduleRunnerPlugin,
    core_pipeline::CorePipelinePlugin,
    diagnostic::FrameCountPlugin,
    log::LogPlugin,
    mesh::MeshPlugin,
    pbr::PbrPlugin,
    prelude::*,
    render::{
        RenderPlugin,
        settings::{RenderCreation, WgpuSettings},
    },
    scene::ScenePlugin,
    state::app::StatesPlugin,
    time::TimePlugin,
    window::ExitCondition,
};

pub trait TestFn: Fn(&mut App) {}
impl<T> TestFn for T where T: Fn(&mut App) {}

#[derive(Debug)]
pub struct TestRunnerPlugin {
    pub timeout: f32,
}
impl Default for TestRunnerPlugin {
    fn default() -> Self {
        Self { timeout: 5. } // 5 sec
    }
}
impl Plugin for TestRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TaskPoolPlugin::default(),
            FrameCountPlugin,
            TimePlugin,
            ScheduleRunnerPlugin::default(),
            LogPlugin::default(),
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Headless test".into(),
                    resizable: true,
                    focused: true,
                    visible: false,
                    desired_maximum_frame_latency: None,
                    ..Default::default()
                }),
                primary_cursor_options: None,
                close_when_requested: true,
                exit_condition: ExitCondition::OnPrimaryClosed,
            },
            AssetPlugin::default(),
            RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: None,
                    ..Default::default()
                }),
                ..Default::default()
            },
            ImagePlugin::default(),
            CorePipelinePlugin,
            MeshPlugin,
            PbrPlugin {
                prepass_enabled: false,
                add_default_deferred_lighting_plugin: false,
                use_gpu_instance_buffer_builder: false,
                ..Default::default()
            },
            StatesPlugin,
            ScenePlugin,
        ));

        let timeout = self.timeout;
        app.add_systems(
            Update,
            move |time: Res<Time<Real>>, mut events: MessageWriter<AppExit>| {
                let elapsed = time.elapsed_secs();
                if elapsed > timeout {
                    error!("Timeout after {elapsed}s");
                    events.write(AppExit::error());
                }
            },
        );
        app.add_systems(PostUpdate, |mut reader: MessageReader<AppExit>| {
            for msg in reader.read() {
                error!("Obtained exit message {msg:?}");
            }
        });
    }
}

#[test]
fn timeout() {
    let mut app = App::new();
    app.add_plugins(TestRunnerPlugin { timeout: 0.5 });
    assert!(app.run().is_error());
}

#[test]
fn explicit_failure() {
    let mut app = App::new();
    app.add_plugins(TestRunnerPlugin::default());
    app.add_systems(First, |mut commands: Commands| {
        commands.write_message(AppExit::error());
    });
    assert!(app.run().is_error());
}

#[test]
fn explicit_success() {
    let mut app = App::new();
    app.add_plugins(TestRunnerPlugin::default());
    app.add_systems(First, |mut commands: Commands| {
        commands.write_message(AppExit::Success);
    });
    assert!(app.run().is_success());
}

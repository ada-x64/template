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

#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub struct TestSuccess(bool);
pub trait TestFn: Fn(&mut App) -> bool {}
impl<T> TestFn for T where T: Fn(&mut App) -> bool {}

/// Runs a headless instance. In order to succesfully exit the app, make sure
/// you send an AppExit event. This can be accomplished with an
/// [`EventWriter`] or with [`World::send_event`]. This does _not_
/// work with [`Commands::trigger`].
///
/// Basic usage:
/// ```rust
/// use bevy::prelude::*;
/// use integration::Runner;
///
/// fn example_test() -> AppExit {
///    Runner::new(|app| {
///        app.add_plugins(MyPlugin);
///        app.add_systems(Update, |mut events: EventWriter<AppExit>| {
///            events.write(AppExit::Success);
///        });
///        app.run()
///    })
///    .run()
/// }
///
/// struct MyPlugin;
/// impl Plugin for MyPlugin {
///    fn build(&self, _: &mut App) {
///        //...
///    }
/// }
/// ```
pub struct Runner {
    test_fn: Box<dyn TestFn>,
    timeout: f32,
}
impl Runner {
    /// Initializes the test runner.
    pub fn new(test: impl TestFn + 'static) -> Self {
        Self {
            test_fn: Box::new(test),
            timeout: 3.,
        }
    }
    /// Sets timeout in seconds.
    pub fn with_timeout(&mut self, timeout: f32) -> &mut Self {
        self.timeout = timeout;
        self
    }
    /// Runs the test.
    pub fn run(&mut self) {
        debug!("Initializing headless app.");
        let mut app = App::new();
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
                // render_creation: RenderCreation::Manual(RenderResources(
                //     RenderDevice::new(WgpuWrapper::new(device)),
                //     RenderQueue(Arc::new(WgpuWrapper::new(queue))),
                //     RenderAdapterInfo(WgpuWrapper::new(adapter_info)),
                //     RenderAdapter(Arc::new(WgpuWrapper::new(adapter))),
                //     RenderInstance(Arc::new(WgpuWrapper::new(instance))),
                // )),
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

        app.init_resource::<TestSuccess>();
        let timeout = self.timeout;
        app.add_systems(
            Update,
            move |time: Res<Time<Real>>,
                  mut events: MessageWriter<AppExit>,
                  mut test_success: ResMut<TestSuccess>| {
                let elapsed = time.elapsed_secs();
                if elapsed > timeout {
                    error!("Timeout after {elapsed}s");
                    events.write(AppExit::error());
                    **test_success = false;
                }
            },
        );

        debug!("Running internal function.");
        let test_result = self.test_fn.as_mut()(&mut app);
        assert!(test_result);
    }
}

#[test]
#[should_panic]
fn timeout() {
    Runner::new(|app| app.run().is_success())
        .with_timeout(0.5)
        .run();
}

#[test]
#[should_panic]
fn explicit_failure() {
    Runner::new(|app| {
        app.add_systems(First, |mut commands: Commands| {
            commands.write_message(AppExit::error());
        })
        .run()
        .is_success()
    })
    .run();
}

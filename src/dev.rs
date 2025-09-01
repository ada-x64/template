use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_minibuffer::prelude::*;
use bevy_minibuffer_inspector as inspector;
use iyes_perf_ui::prelude::{PerfUiDefaultEntries, PerfUiRoot};

pub struct DevPlugin;

fn inspect_perf(
    mut minibuffer: Minibuffer,
    mut commands: Commands,
    perf_ui: Query<Entity, With<PerfUiRoot>>,
) {
    if let Ok(entity) = perf_ui.single() {
        commands.entity(entity).despawn();
    } else {
        commands.spawn(PerfUiDefaultEntries::default());
    }
    minibuffer.clear()
}

fn diagnostics_plugin(app: &mut App) {
    app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
        .add_plugins(iyes_perf_ui::PerfUiPlugin);
}
fn minibuffer_plugin(app: &mut App) {
    app.add_plugins((
        MinibufferPlugins,
        bevy_mod_debugdump::CommandLineArgs,
        // TODO: Switch to official hotloading in 0.17
        bevy_simple_subsecond_system::SimpleSubsecondPlugin::default(),
    ))
    .add_plugins(EguiPlugin::default())
    .add_acts((
        BasicActs::default(),
        inspector::WorldActs::default().configure("inspect_world", |act| {
            act.bind([KeyCode::F1]);
        }),
        // TODO: ADD RESOURCES HERE
        inspector::ResourceActs::default().configure("inspect_resource", |act| {
            act.bind([KeyCode::F2]);
        }),
        // TODO: ADD STATES HERE
        inspector::StateActs::default().configure("inspect_state", |act| {
            act.bind([KeyCode::F3]);
        }),
        inspector::AssetActs::default()
            .configure("inspect_asset", |act| {
                act.bind([KeyCode::F4]);
            })
            .add::<StandardMaterial>(),
        inspector::FilterQueryActs::default()
            .configure("inspect_filter_query", |act| {
                act.bind([KeyCode::F5]);
            })
            .add::<With<Transform>>()
            .add::<With<Mesh3d>>(),
        Act::new(inspect_perf).bind([KeyCode::F12]),
    ))
    .add_systems(Startup, |mut minibuffer: Minibuffer| {
        minibuffer.message("Dev mode enabled.");
        minibuffer.set_visible(true);
    });
}

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        info!("Dev plugin!");
        app.add_plugins((diagnostics_plugin, minibuffer_plugin));
    }
}

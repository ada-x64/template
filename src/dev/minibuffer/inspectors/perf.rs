// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use bevy::prelude::*;
use bevy_minibuffer::prelude::*;
use iyes_perf_ui::prelude::{PerfUiDefaultEntries, PerfUiRoot};

pub fn inspect_perf(
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

pub fn plugin(app: &mut App) {
    app.add_plugins((
        bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
        bevy::diagnostic::EntityCountDiagnosticsPlugin,
        bevy::diagnostic::SystemInformationDiagnosticsPlugin,
        bevy::render::diagnostic::RenderDiagnosticsPlugin,
        iyes_perf_ui::PerfUiPlugin,
    ))
    .add_acts((Act::new(inspect_perf).bind([KeyCode::F12]),));
}

pub(crate) mod actions;
pub(crate) mod inspectors;

use bevy::prelude::*;
use bevy_flycam::FlyCam;
use bevy_minibuffer::prelude::*;

pub fn plugin(app: &mut App) {
    // third-party
    app.add_plugins((
        bevy_inspector_egui::bevy_egui::EguiPlugin {
            enable_multipass_for_primary_context: true,
        },
        bevy_inspector_egui::DefaultInspectorConfigPlugin,
        MinibufferPlugins,
        bevy_mod_debugdump::CommandLineArgs,
        // TODO: Switch to official hotloading in 0.17
        bevy_simple_subsecond_system::SimpleSubsecondPlugin::default(),
        // temp?
        bevy_flycam::NoCameraPlayerPlugin,
    ));
    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn((
            Camera3d::default(),
            Camera {
                is_active: false,
                ..Default::default()
            },
            FlyCam,
            Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));
    });

    // local
    app.add_plugins((inspectors::plugin, actions::plugin));
}

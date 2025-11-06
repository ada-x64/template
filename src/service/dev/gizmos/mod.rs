use crate::prelude::*;
use bevy::render::view::RenderLayers;

mod data;
mod systems;

pub mod prelude {
    pub use super::data::*;
}

pub fn plugin(app: &mut App) {
    app.add_plugins(systems::plugin);

    app.add_plugins(PhysicsDebugPlugin::default())
        .insert_gizmo_config(
            PhysicsGizmos::default(),
            GizmoConfig {
                render_layers: RenderLayers::from(RenderLayer::GIZMOS_3D),
                enabled: true,
                ..Default::default()
            },
        )
        .insert_gizmo_config(
            LightGizmoConfigGroup::default(),
            GizmoConfig {
                render_layers: RenderLayers::from(RenderLayer::GIZMOS_3D),
                enabled: false,
                ..Default::default()
            },
        )
        .insert_gizmo_config(
            CameraGizmoConfigGroup::default(),
            GizmoConfig {
                render_layers: RenderLayers::from(RenderLayer::GIZMOS_3D),
                enabled: true,
                line: GizmoLineConfig {
                    style: GizmoLineStyle::Dashed {
                        gap_scale: 1.,
                        line_scale: 1.,
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
        );
}

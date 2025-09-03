use avian3d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::{prelude::*, render::view::RenderLayers};

use crate::data::RenderLayer;

pub fn plugin(app: &mut App) {
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
        );
}

use crate::data::{CameraOrder, RenderLayer};
use bevy::{prelude::*, render::view::RenderLayers};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct UiCamera;

fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("UI Camera"),
        UiCamera,
        Camera2d,
        IsDefaultUiCamera,
        Camera {
            order: CameraOrder::Ui.into(),
            ..default()
        },
        RenderLayers::from(RenderLayer::DEFAULT),
    ));
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_ui_camera);
    app.register_type::<UiCamera>();
}

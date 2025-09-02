use crate::data::CameraOrder;
use bevy::prelude::*;

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
    ));
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_ui_camera);
    app.register_type::<UiCamera>();
}

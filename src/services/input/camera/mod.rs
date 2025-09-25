use crate::prelude::*;

pub(crate) mod data;
pub(crate) mod fly;
pub(crate) mod tracking;

fn update_camera_controller(trigger: Trigger<InsertCameraController>, mut commands: Commands) {
    commands
        .entity(trigger.entity)
        .insert(trigger.new_controller);
}

pub fn plugin(app: &mut App) {
    app.add_plugins((tracking::plugin, fly::plugin));
    app.add_observer(update_camera_controller);
}

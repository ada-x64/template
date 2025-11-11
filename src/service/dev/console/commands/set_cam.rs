use bevy_console::reply;
use clap::Parser;

use crate::prelude::*;

#[derive(Parser, ConsoleCommand)]
#[command(name = "set_cam")]
pub struct SetCamCmd {
    #[arg(value_enum)]
    cam: CameraControllerKind,
}

fn set_cam_cmd(
    mut log: ConsoleCommand<SetCamCmd>,
    cams: Query<(Entity, &CameraController)>,
    mut commands: Commands,
) {
    if let Some(Ok(cmd)) = log.take() {
        reply!(log, "Switching to {:?}", cmd.cam);
        for (cam, controller) in cams {
            commands
                .entity(cam)
                .insert(controller.with_active(controller.kind == cmd.cam));
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_console_command::<SetCamCmd, _>(set_cam_cmd);
}

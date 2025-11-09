use bevy_console::reply;
use clap::{Parser, ValueEnum};

use crate::prelude::*;

#[derive(ValueEnum, Clone, Copy, Debug)]
enum Cameras {
    Fly,
    Tracking,
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "set_cam")]
pub struct SetCamCmd {
    #[arg(value_enum)]
    cam: Cameras,
}

fn set_cam_cmd(mut log: ConsoleCommand<SetCamCmd>) {
    if let Some(Ok(cmd)) = log.take() {
        reply!(log, "Switching to {:?}", cmd.cam);
        // set the current camera using an event
        match cmd.cam {
            Cameras::Fly => {
                // set the current camera using an event
            }
            Cameras::Tracking => {
                // ...
            }
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_console_command::<SetCamCmd, _>(set_cam_cmd);
}

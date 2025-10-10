// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------

pub mod camera;
pub mod cursor;
pub mod data;

use bevy::{prelude::*, window::CursorGrabMode};
use bevy_enhanced_input::prelude::*;

pub mod prelude {
    pub use super::camera::prelude::*;
    pub use super::data::*;
}
pub use prelude::*;

pub fn spawn_global_ctx(mut commands: Commands) {
    commands.spawn((
        ICtxGlobal,
        ContextActivity::<ICtxGlobal>::ACTIVE,
        ContextPriority::<ICtxGlobal>::new(1000),
        actions![
            ICtxGlobal[(
                Action::<PAQuit>::new(),
                bindings![KeyCode::Escape],
                ActionSettings {
                    consume_input: false,
                    require_reset: true,
                    ..Default::default()
                }
            )]
        ],
    ));
}

pub fn exit_app(_: Trigger<Completed<PAQuit>>, mut commands: Commands, win: Single<&Window>) {
    info!("exit_app");
    if matches!(win.cursor_options.grab_mode, CursorGrabMode::None) {
        commands.send_event(AppExit::Success);
    }
}

pub fn plugin(app: &mut App) {
    app.add_plugins((cursor::plugin, camera::plugin))
        .add_input_context::<ICtxGlobal>()
        .add_observer(exit_app)
        .add_systems(Startup, spawn_global_ctx);
}

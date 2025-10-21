use crate::prelude::*;

fn spawn_global_ctx(mut commands: Commands) {
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

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_global_ctx);
}

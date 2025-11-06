use crate::prelude::*;

/// Always-active input context
#[derive(Component)]
pub struct ICtxGlobal;

#[derive(InputAction)]
#[action_output(bool)]
pub struct PAQuit;

#[derive(Event)]
pub struct SpawnGlobalCtx;

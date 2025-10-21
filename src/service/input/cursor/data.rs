use crate::prelude::*;

/// Enables cursor capture
#[derive(Component)]
pub struct ICtxCaptureCursor;

#[derive(InputAction)]
#[action_output(bool)]
pub struct PACaptureCursor;

#[derive(InputAction)]
#[action_output(bool)]
pub struct PAReleaseCursor;

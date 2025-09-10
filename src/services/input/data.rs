// ------------------------------------------
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------
use crate::prelude::*;

/// Always-active input context
#[derive(Component)]
pub struct ICtxGlobal;

#[derive(InputAction)]
#[action_output(bool)]
pub struct PAQuit;

/// Enables cursor capture
#[derive(Component)]
pub struct ICtxCaptureCursor;

#[derive(InputAction)]
#[action_output(bool)]
pub struct PACaptureCursor;

#[derive(InputAction)]
#[action_output(bool)]
pub struct PAReleaseCursor;

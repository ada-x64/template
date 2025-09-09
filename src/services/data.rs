use bevy::prelude::*;

#[derive(Event)]
pub struct GrabCursor<const GRAB: bool>;

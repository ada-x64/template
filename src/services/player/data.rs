//! Contains the states and constants for the PlayerService plugin
pub use bevy::prelude::*;

pub const PLAYER_CAPSULE_HEIGHT: f32 = 0.5;
pub const PLAYER_CAPSULE_RADIUS: f32 = 0.5;

#[derive(Event, Reflect, Copy, Clone, Debug)]
pub struct SpawnPlayerRoot;

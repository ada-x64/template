//! Contains the states and constants for the PlayerService plugin
pub use bevy::prelude::*;

pub const PLAYER_CAPSULE_HEIGHT: f32 = 3.;
pub const PLAYER_CAPSULE_RADIUS: f32 = 0.5;

#[derive(SystemSet, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlayerSystems;

#[derive(Event, Reflect, Copy, Clone, Debug)]
pub struct SpawnPlayerRoot;

#[derive(Component, Debug, Default)]
pub struct PlayerCam;

#[derive(Component)]
pub struct PlayerRoot;

#[derive(Component)]
pub struct PlayerController;

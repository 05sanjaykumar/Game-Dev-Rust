use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Platform;

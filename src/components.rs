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

#[derive(Component)]
pub struct PlayerState {
    pub on_ground: bool,
    pub coyote_timer: f32,
    pub jump_buffer_timer: f32,
}
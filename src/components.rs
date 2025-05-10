use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
#[derive(Debug)]
pub struct Velocity {
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

#[derive(Component)]
pub struct MovingPlatform {
    pub direction: Vec2,
    pub speed: f32,
    pub range: f32,
    pub origin: Vec3,
}
use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;

use crate::components::{Player, Velocity, PlayerState};

pub fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity, &mut PlayerState), With<Player>>,
    time: Res<Time>,
) {
    const SPEED: f32 = 300.0;
    const JUMP_FORCE: f32 = 100.0*6.0;
    const GRAVITY: f32 = -(9.8*100.0);
    const COYOTE_TIME: f32 = 0.15; // seconds
    const JUMP_BUFFER_TIME: f32 = 0.15; // seconds
    const FAST_FALL_GRAVITY: f32 = -1600.0;
    const MAX_FALL_SPEED: f32 = -600.0;


    for (mut transform, mut velocity, mut player_state) in &mut query {

        let dt = time.delta_secs();

        //Movement and control
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Space) && velocity.y == 0.0 {
            velocity.y = JUMP_FORCE;
        }

        transform.translation.x += direction.x * SPEED * dt;

        // Gravity
        let gravity = if velocity.y> 0.0 && !keyboard_input.pressed(KeyCode::Space){
            FAST_FALL_GRAVITY
        }else{
            GRAVITY
        };

        velocity.y += gravity * dt;

        // Clamp fall speed
        if velocity.y < MAX_FALL_SPEED {
            velocity.y = MAX_FALL_SPEED;
        }
        transform.translation.y += velocity.y * dt;

        // --- Check if touching ground ---

        if transform.translation.y <= 0.0 {
            transform.translation.y = 0.0;
            velocity.y = 0.0;
            player_state.on_ground = true;
            player_state.coyote_timer = COYOTE_TIME;
        } else {
            player_state.on_ground = false;
            player_state.coyote_timer -= dt;
        }

        // --- Jump Buffer ---
        if keyboard_input.just_pressed(KeyCode::Space) {
            player_state.jump_buffer_timer = JUMP_BUFFER_TIME;
        } else {
            player_state.jump_buffer_timer -= dt;
        }

        // --- Jump Conditions ---
        if (player_state.on_ground || player_state.coyote_timer > 0.0) && player_state.jump_buffer_timer > 0.0 {
            velocity.y = JUMP_FORCE;
            player_state.coyote_timer = 0.0;
            player_state.jump_buffer_timer = 0.0;
        }

    }
}
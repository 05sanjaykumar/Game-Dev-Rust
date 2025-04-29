use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;

use crate::components::{Player, Velocity, PlayerState};

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.7, 0.9),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -200.0, 0.0),
        Visibility::Visible,
        Player,
        Velocity { x: 0.0, y: 0.0 },
        PlayerState {
            on_ground: true,
            coyote_timer: 0.0,
            jump_buffer_timer: 0.0,
        },

    ));
}

pub fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity, &mut PlayerState), With<Player>>,
    time: Res<Time>,
) {
    const SPEED: f32 = 300.0;
    const JUMP_FORCE: f32 = 100.0*3.0;
    const GRAVITY: f32 = -(9.8*20.0);
    const COYOTE_TIME: f32 = 0.15; // seconds
    const JUMP_BUFFER_TIME: f32 = 0.15; // seconds


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
        velocity.y += GRAVITY * dt;
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

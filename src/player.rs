use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;

use crate::components::{Player, Velocity};

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
    ));
}

pub fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    const SPEED: f32 = 300.0;
    const JUMP_FORCE: f32 = 100.0;
    const GRAVITY: f32 = -9.8;

    for (mut transform, mut velocity) in &mut query {
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

        transform.translation.x += direction.x * SPEED * time.delta().as_secs_f32();
        velocity.y += GRAVITY * time.delta().as_secs_f32();
        transform.translation.y += velocity.y * time.delta().as_secs_f32();

        if transform.translation.y < 0.0 {
            transform.translation.y = 0.0;
            velocity.y = 0.0;
        }
    }
}

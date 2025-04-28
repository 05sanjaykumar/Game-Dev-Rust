use bevy::prelude::*;
use crate::components::{Platform, Velocity, Player};

pub fn spawn_platforms(mut commands: Commands) {
    // Ground
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(400.0, 30.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Visibility::Visible,
        Platform,
    ));

    // Floating
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.647, 0.0),
            custom_size: Some(Vec2::new(200.0, 30.0)),
            ..default()
        },
        Transform::from_xyz(200.0, 150.0, 0.0),
        Visibility::Visible,
        Platform,
    ));
}

pub fn platform_collision_system(
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    platform_query: Query<&Transform, (With<Platform>, Without<Player>)>,
) {

    for (mut player_transform, mut velocity) in &mut player_query {
        for platform_transform in &platform_query {
            
            let player_pos = player_transform.translation;
            let platform_pos = platform_transform.translation;

            let player_size = Vec2::new(50.0, 50.0);
            let platform_size = Vec2::new(200.0, 30.0); 

            let collision_x = (player_pos.x - platform_pos.x).abs() < (player_size.x + platform_size.x) / 2.0;
            let collision_y = (player_pos.y - platform_pos.y).abs() < (player_size.y + platform_size.y) / 2.0;

            if collision_x && collision_y && velocity.y <= 0.0 {
                player_transform.translation.y = platform_pos.y + (platform_size.y + player_size.y) / 2.0;
                velocity.y = 0.0;
            }
        }
    }
}
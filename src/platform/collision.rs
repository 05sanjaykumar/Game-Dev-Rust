use bevy::prelude::*;
use crate::components::{Platform, Velocity, Player};

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

            let from_above = player_pos.y > platform_pos.y;

            if collision_x && collision_y && velocity.y <= 0.0 && from_above {
                player_transform.translation.y = platform_pos.y + (platform_size.y + player_size.y) / 2.0;
                velocity.y = 0.0;
            }
        }
    }
}
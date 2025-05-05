use bevy::prelude::*;
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
        Velocity { y: 0.0 },
        PlayerState {
            on_ground: true,
            coyote_timer: 0.0,
            jump_buffer_timer: 0.0,
        },

    ));
}

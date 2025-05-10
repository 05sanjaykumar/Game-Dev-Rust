use bevy::prelude::*;
use crate::components::Platform;

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

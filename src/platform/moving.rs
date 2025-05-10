use bevy::prelude::*;
use crate::components::Platform;
use crate::components::MovingPlatform;

pub fn spawn_moving_platforms(mut commands:Commands){

    let size = Vec2::new(150.0,200.0);
    let start_pos = Vec3::new(-200.0, 100.0, 0.0);

    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.5, 0.0),
            custom_size: Some(size),
            ..default()
        },
        Transform::from_translation(start_pos),
        Visibility::Visible,
        Platform,
        MovingPlatform {
            direction: Vec2::X, // move horizontally
            speed: 100.0,
            range: 150.0,
            origin: start_pos,
        },
    ));
}

pub fn moving_platform_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &MovingPlatform)>,
){
    for(mut transform, mover) in &mut query{
        let t = time.elapsed_secs();
        let offset = (t * mover.speed).sin() * mover.range;
        let movement = mover.direction * offset;

        transform.translation = mover.origin + movement.extend(0.0);

    }
}
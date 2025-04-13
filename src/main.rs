use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
// use bevy::math::Vec2;

#[derive(Component)]
struct Player;

#[derive(Component, Debug)]
struct Velocity {
    pub x: f32,
    pub y: f32,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement_system,camera_follow_system))
        .run();
}

fn camera_follow_system(player_query: Query<&Transform, With<Player>>,mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}

fn setup(mut commands: Commands) {
    // Spawn 2D camera
    commands.spawn(Camera2d);

    // Spawn player sprite
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

fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    const SPEED: f32 = 300.0;
    const JUMP_FORCE: f32 = 400.0;
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
            velocity.y = JUMP_FORCE;  // Jumping logic
        }

        // Horizontal movement
        transform.translation.x += direction.x * SPEED * time.delta().as_secs_f32();
        
        // Gravity + vertical motion
        velocity.y += GRAVITY * time.delta().as_secs_f32();
        transform.translation.y += velocity.y * time.delta().as_secs_f32();

        // Floor clamp
        if transform.translation.y < 0.0 {
            transform.translation.y = 0.0;
            velocity.y = 0.0;
        }
    }
}

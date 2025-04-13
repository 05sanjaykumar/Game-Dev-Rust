use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
// use bevy::math::Vec2;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement_system)
        .run();
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
    ));
}

fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    const SPEED: f32 = 300.0;
    for mut transform in &mut query {
        let mut direction = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        
        transform.translation.x += direction.x * SPEED * time.delta().as_secs_f32();
    }
}

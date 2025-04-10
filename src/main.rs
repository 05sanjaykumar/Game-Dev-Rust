use bevy::prelude::*;

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
    // Camera
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.7, 0.9),
                custom_size: Some(Vec2::new(50.0, 50.0)),  // size of player square
                ..default()
            },
            transform: Transform::from_xyz(0.0, -200.0, 0.0),  // initial position
            ..default()
        },
        Player, // add marker component
    ));
}


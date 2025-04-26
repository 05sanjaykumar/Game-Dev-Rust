use bevy::prelude::*;

mod player;
mod platform;
mod camera;
mod components;

use player::player_movement_system;
use platform::spawn_platforms;
use camera::camera_follow_system;
use player::spawn_player;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (camera::setup_camera, spawn_player, spawn_platforms))
        .add_systems(Update, (player_movement_system, platform_collision_system,camera_follow_system))
        .run();
}

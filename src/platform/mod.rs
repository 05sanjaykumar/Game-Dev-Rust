mod spawn;
mod collision;
pub mod moving;


pub use spawn::spawn_platforms;
pub use collision::platform_collision_system;
pub use moving::{spawn_moving_platforms, moving_platform_system};

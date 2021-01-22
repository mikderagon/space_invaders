pub mod bullet;
mod play_state;

use amethyst::ecs::prelude::World;

pub use self::bullet::BulletResource;
pub use self::play_state::PlayState;

/// Add all the resources needed at the start to the world
/// Note that [laserResource] is not added here, but when the laser component is created.
pub fn add_resources(world: &mut World) {
  world.insert(PlayState { lives: 3 });
}

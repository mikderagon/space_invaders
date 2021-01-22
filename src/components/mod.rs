use amethyst::ecs::{WorldExt, prelude::World};

mod ship;
mod bullet;
mod alien;

pub use self::ship::Ship;
pub use self::bullet::Bullet;
pub use self::ship::Side;
pub use self::alien::Alien;

pub fn register_components(world: &mut World) {
  world.register::<Alien>();
  world.register::<Ship>();
  world.register::<Bullet>();
}
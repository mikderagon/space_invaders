use amethyst::{
  core::timing::Time,
  core::transform::Transform,
  core::ecs::ReadExpect,
  derive::SystemDesc,
  ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
  window::ScreenDimensions,
};

use crate::components::Bullet;
use crate::config::GAME_CONFIGURATION;

#[derive(SystemDesc)]
pub struct BulletSystem;

impl<'s> System<'s> for BulletSystem {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, Bullet>,
    WriteStorage<'s, Transform>,
    Read<'s, Time>,
    ReadExpect<'s, ScreenDimensions>,
  );

  fn run(&mut self, (
    entities,
    bullets,
    mut transforms,
    time,
    screen_dimensions
  ): Self::SystemData) {
    for (bullet_entity, _bullet_component, transform) in (&entities, &bullets, &mut transforms).join() {
      transform.prepend_translation_y(GAME_CONFIGURATION.bullet_velocity * time.delta_seconds());
    }
  }
}
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Entities, Join, Read, System, WriteStorage};
use rand::thread_rng;

use crate::components::Alien;
use crate::entities::alien;

pub struct AlienSystem;

impl<'s> System<'s> for AlienSystem {
  type SystemData = (
    Entities<'s>,
    WriteStorage<'s, Alien>,
    WriteStorage<'s, Transform>,
    Read<'s, Time>,
  );

  fn run(&mut self, (entities, mut aliens, mut transforms, time): Self::SystemData) {
    for (alien_entity, alien_component, alien_transform) in (&*entities, &aliens, &transforms).join()
    {
      if alien_component.is_killed
      {
        let _res = entities.delete(alien_entity);
      }
    }
  }
}
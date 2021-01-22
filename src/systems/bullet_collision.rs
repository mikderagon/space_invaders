use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Entities, Join, ReadStorage, System, WriteStorage};

use crate::components::Alien;
use crate::components::Bullet;

pub struct BulletCollisionSystem;

impl <'s> System<'s> for BulletCollisionSystem {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, Bullet>,
    ReadStorage<'s, Transform>,
    WriteStorage<'s, Alien>,
  );

  fn run(&mut self, (entities, bullets, transforms, mut aliens): Self::SystemData) {
    for (bullet_entity, bullet_component, bullet_transform) in (&*entities, &bullets, &transforms).join()
    {
      let bullet_left = bullet_transform.translation()[0] - (bullet_component.width / 2.);
      let bullet_right = bullet_left + bullet_component.width;
      let bullet_top = bullet_transform.translation()[1] + (bullet_component.height / 2.);

      // println!("left: {}, right: {}, top: {}", bullet_left, bullet_right, bullet_top);
      for (alien_component, alien_transform) in (&mut aliens, &transforms).join()
      {
        let alien_left = alien_transform.translation()[0] - (alien_component.width / 2.);
        let alien_bottom = alien_transform.translation()[1] - (alien_component.height / 2.);
        let alien_right = alien_left + alien_component.width;

        // println!("left: {}, bottom: {}, right: {}", alien_left, alien_bottom, alien_right);        
        
        if (bullet_left >= alien_left && bullet_right <= alien_right)
            && (bullet_top - 200. >= alien_bottom)
        {
          let _result = entities.delete(bullet_entity);
          alien_component.is_killed = true;
        }
      }
    }
  }
}
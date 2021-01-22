use crate::components::Ship;
use crate::config::GAME_CONFIGURATION;
use crate::entities::bullet::fire_bullet;
use crate::resources::BulletResource;

use amethyst::core::math::Vector3;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Entities, Join, LazyUpdate, ReadExpect, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::input::StringBindings;
use amethyst::window::ScreenDimensions;

use crate::components::Side;

pub struct ShipSystem;

impl<'s> System<'s> for ShipSystem {
  type SystemData = (
    Entities<'s>,
    WriteStorage<'s, Ship>,
    WriteStorage<'s, Transform>,
    ReadExpect<'s, Time>,
    ReadExpect<'s, InputHandler<StringBindings>>,
    ReadExpect<'s, BulletResource>,
    ReadExpect<'s, LazyUpdate>,
  );

  fn run(&mut self, (
    entities,
    mut ships,
    mut transforms,
    time,
    input,
    bullet_resource,
    lazy_update,
  ): Self::SystemData) {
    for (ship, transform) in (&mut ships, &mut transforms).join() {
      if ship.trigger_reset_timer > 0. {
        ship.trigger_reset_timer -= time.delta_seconds();
      }

      let optional_movement = input.axis_value("ship");
      let optional_action = input.action_is_down("fire");

      if let Some(action) = optional_action {
        if action && ship.trigger_reset_timer <= 0. {
          match ship.bullet_side {
            Side::Left => {
              ship.bullet_side = Side::Right;
              let fire_position = Vector3::new(
                transform.translation()[0],
                transform.translation()[1] + (ship.height * 0.2),
                0.,
              );
              fire_bullet(&entities, &bullet_resource, fire_position, &lazy_update);
    
              ship.trigger_reset_timer = GAME_CONFIGURATION.trigger_reset_timeout;    
            },
            Side::Right => {
              ship.bullet_side = Side::Left;
              let fire_position = Vector3::new(
                transform.translation()[0] + ship.width * 1.,
                transform.translation()[1] + (ship.height * 0.2),
                0.,
              );
              fire_bullet(&entities, &bullet_resource, fire_position, &lazy_update);
    
              ship.trigger_reset_timer = GAME_CONFIGURATION.trigger_reset_timeout;    
            },
          }
        }
      }
      
      if let Some(mv_amount) = optional_movement {
        let scaled_amount = 15. * mv_amount as f32;
        transform.prepend_translation_x(scaled_amount);
      }


      if transform.translation()[0] < 0. {
        transform.set_translation_x(0.);
        ship.velocity = -ship.velocity;
      } else if transform.translation()[0] >= 1800. {
        transform.set_translation_x(1800.);
        ship.velocity = -ship.velocity;
      }
    }
  }
}
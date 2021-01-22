use amethyst::assets::Handle;
use amethyst::core::math::Vector3;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Entity, World, WorldExt};
use amethyst::prelude::Builder;
use amethyst::renderer::SpriteRender;
use amethyst::renderer::SpriteSheet;
use rand::{thread_rng, Rng, ThreadRng};

use amethyst::window::ScreenDimensions;
use crate::config::GAME_CONFIGURATION;
use crate::components::Alien;

pub fn initialise_aliens(
  world: &mut World,
  sprite_sheet_handle: Handle<SpriteSheet>,
) -> Vec<Entity> {
  let alien = Alien {
    velocity: GAME_CONFIGURATION.alien_velocity,
    width: 541.,
    height: 513.,
    is_killed: false,
  };
  
  let numbers = 0..;
  let range = numbers.take(10);
  range.map(|_number| {
    let _screen_dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
    let (screen_width, screen_height) = { (_screen_dimensions.width(), _screen_dimensions.height()) };
    let mut local_transform = Transform::default();
    local_transform.set_scale(Vector3::new(0.2, 0.2, 0.));
    local_transform.set_translation_xyz(
      screen_width * 0.1 * ((_number as i8 % 8) as f32) + screen_width * 0.15,
      screen_height * 0.9 - (((_number / 8) as f32).floor() * 200.),
      0.,
    );

    world.create_entity()
      .with(alien.clone())
      .with(local_transform)
      .with(SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
      })
      .build()
  })
  .collect()
}
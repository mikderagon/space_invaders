use amethyst::{
  core::{math::Vector3, transform::Transform},
  ecs::prelude::{Entity, World, WorldExt},
  prelude::Builder,
  assets::Handle,
  renderer::{SpriteSheet, SpriteRender},
  window::ScreenDimensions,
};

pub const SHIP_HEIGHT: f32 = 200.;
pub const SHIP_WIDTH: f32 = 140.;

use crate::components::Ship;
use crate::components::Side;

pub fn initialise_ship(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) -> Entity {
  let _screen_dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
  let sprite_render = SpriteRender::new(sprite_sheet_handle, 4);

  let mut transform = Transform::default();

  transform.set_scale(Vector3::new(0.3, 0.3, 0.));
  transform.set_translation_xyz(_screen_dimensions.width() / 2., SHIP_HEIGHT * 0.5, 0.);

  world.create_entity()
    .with(sprite_render)
    .with(Ship {
      velocity: 0.,
      width: SHIP_WIDTH,
      height: SHIP_HEIGHT,
      trigger_reset_timer: 0.,
      bullet_side: Side::Left,
    })
    .with(transform)
    .build()
}
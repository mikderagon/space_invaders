use amethyst::{
  assets::{AssetStorage, Loader, Handle},
  core::math::Vector3,
  core::transform::Transform,
  ecs::{Component, DenseVecStorage},
  prelude::*,
  renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::components::register_components;
use crate::entities::initialise_entities;
use crate::resources::add_resources;

#[derive(Default)]
pub struct GameState;

impl SimpleState for GameState {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;

    register_components(world);
    add_resources(world);
    initialise_entities(world);
  }
}
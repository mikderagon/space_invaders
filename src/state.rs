use amethyst::{
  assets::{AssetStorage, Loader, Handle},
  core::math::Vector3,
  core::transform::Transform,
  ecs::{Component, DenseVecStorage, Entity},
  prelude::*,
  renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
  ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
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
    initialise_scoreboard(world);
  }
}

#[derive(Default)]
pub struct ScoreBoard {
  pub aliens_killed: i32,
}

pub struct ScoreText {
  pub aliens_killed: Entity,
}

fn initialise_scoreboard(world: &mut World) {
  let font = world.read_resource::<Loader>().load(
    "font/square.ttf",
    TtfFormat,
    (),
    &world.read_resource(),
  );

  let count_transform = UiTransform::new(
    "ALIENS".to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
    -50., -50., 1., 200., 50.,
  );

  let aliens_killed = world.create_entity()
    .with(count_transform)
    .with(UiText::new(
      font.clone(),
      "0".to_string(),
      [1., 1., 1., 1.,],
      50.,
      LineMode::Single,
      Anchor::Middle,
    ))
    .build();

  world.insert(ScoreText { aliens_killed })
}
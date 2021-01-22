use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Clone)]
pub struct Alien {
  pub velocity: f32,
  pub width: f32,
  pub height: f32,
  pub is_killed: bool,
}

impl Component for Alien {
  type Storage = DenseVecStorage<Self>;
}
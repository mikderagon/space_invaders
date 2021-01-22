use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Clone)]
pub struct Bullet {
  pub velocity: f32,
  pub width: f32,
  pub height: f32,
}

impl Component for Bullet {
  type Storage = DenseVecStorage<Self>;
}
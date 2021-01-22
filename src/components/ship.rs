use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub enum Side {
  Left,
  Right,
}

pub struct Ship {
  pub velocity: f32,
  pub height: f32,
  pub width: f32,
  // How much time in seconds has passed since the last time the laser was fired
  pub trigger_reset_timer: f32,
  pub bullet_side: Side,
}

impl Component for Ship {
  type Storage = DenseVecStorage<Self>;
}
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Entities, Join, Read, ReadExpect, Write, System, WriteStorage};
use amethyst::ui::UiText;
use rand::thread_rng;

use crate::components::Alien;
use crate::entities::alien;

use crate::state::{ScoreBoard, ScoreText};

pub struct AlienSystem;

impl<'s> System<'s> for AlienSystem {
  type SystemData = (
    Entities<'s>,
    WriteStorage<'s, Alien>,
    WriteStorage<'s, Transform>,
    Read<'s, Time>,
    WriteStorage<'s, UiText>,
    Write<'s, ScoreBoard>,
    ReadExpect<'s, ScoreText>,
  );

  fn run(&mut self, (
    entities,
    mut aliens,
    mut transforms,
    time,
    mut ui_text,
    mut aliens_killed,
    aliens_killed_text,
  ): Self::SystemData) {
    for (alien_entity, alien_component, alien_transform) in (&*entities, &aliens, &transforms).join()
    {
      if alien_component.is_killed
      {
        let _res = entities.delete(alien_entity);       
        aliens_killed.aliens_killed = (aliens_killed.aliens_killed + 1).min(999);

        if let Some(text) = ui_text.get_mut(aliens_killed_text.aliens_killed) {
          text.text = aliens_killed.aliens_killed.to_string();
        }
      }
    }
  }
}
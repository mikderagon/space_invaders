use amethyst::core::math::Vector3;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Entities, Entity, LazyUpdate, ReadExpect, World};

use crate::components::Bullet as BulletComponent;
use crate::config::GAME_CONFIGURATION;
use crate::resources::bullet::BulletResource;

use amethyst::assets::Handle;
use amethyst::renderer::SpriteRender;
use amethyst::renderer::SpriteSheet;

pub fn initialise_bullet_resource(
  world: &mut World,
  sprite_sheet_handle: Handle<SpriteSheet>,
) -> BulletResource {
  let bullet_resource = BulletResource {
    component: BulletComponent {
      velocity: GAME_CONFIGURATION.bullet_velocity,
      width: 140.,
      height: 528.,
    },
    sprite_render: SpriteRender {
      sprite_sheet: sprite_sheet_handle.clone(),
      sprite_number: 1,
    },
  };
  world.insert(bullet_resource.clone());
  bullet_resource
}

pub fn fire_bullet(
  entities: &Entities,
  bullet_resource: &ReadExpect<BulletResource>,
  fire_position: Vector3<f32>,
  lazy_update: &ReadExpect<LazyUpdate>,
) {
  let bullet_entity: Entity = entities.create();
  let local_transform = {
    let mut local_transform = Transform::default();
    local_transform.set_scale(Vector3::new(0.1, 0.1, 0.));
    local_transform.set_translation(fire_position);
    let p = local_transform.translation()[0];
    local_transform.set_translation_x(p - (bullet_resource.component.width / 2.0));
    local_transform
  };
  lazy_update.insert(bullet_entity, bullet_resource.component.clone());
  lazy_update.insert(bullet_entity, bullet_resource.sprite_render.clone());
  lazy_update.insert(bullet_entity, local_transform);
}
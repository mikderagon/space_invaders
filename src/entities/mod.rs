pub mod bullet;
pub mod ship;
pub mod camera;
pub mod alien;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::ecs::prelude::World;
use amethyst::ecs::prelude::WorldExt;
use amethyst::renderer::formats::texture::ImageFormat;
use amethyst::renderer::SpriteSheet;
use amethyst::renderer::SpriteSheetFormat;
use amethyst::renderer::Texture;

pub use self::bullet::fire_bullet;

/// Initialises all the entities (some are just set up as resources so the entities can be created later on demand)
pub fn initialise_entities(world: &mut World) {
    // Load the sprite sheet with all our entities
    let sprite_sheet_handle = {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "spritesheet.png",
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "spritesheet.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sprite_sheet_store,
        )
    };
    alien::initialise_aliens(world, sprite_sheet_handle.clone());
    ship::initialise_ship(world, sprite_sheet_handle.clone());
    camera::initialise_camera(world);
    bullet::initialise_bullet_resource(world, sprite_sheet_handle.clone());
}
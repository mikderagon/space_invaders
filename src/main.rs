use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
};

mod state;
mod systems;
mod config;
pub mod components;
pub mod resources;
pub mod entities;

pub use crate::config::GAME_CONFIGURATION;
pub use crate::state::GameState;

const BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub fn run() -> Result<(), amethyst::Error> {
    let _ = &config::GAME_CONFIGURATION;

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");

    let display = config_dir.join("display.ron");
    let bindings = config_dir.join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(bindings)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display)?.with_clear(BACKGROUND_COLOR))
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::ShipSystem, "player_system", &["input_system"])
        .with(systems::AlienSystem, "alien_system", &[])
        .with(systems::BulletSystem, "bullet_system", &[])
        .with(systems::BulletCollisionSystem, "bullet_collision_system", &[]);

    let mut game = Application::new(assets_dir, GameState, game_data)?;

    Ok(game.run())
}

pub fn main() {
    amethyst::start_logger(Default::default());
    if let Err(e) = run() {
        println!("Error occured during game execution: {}", e);
        ::std::process::exit(1);
    }
}
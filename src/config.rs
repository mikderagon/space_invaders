//! The set of items used by the Game Designer to tune the game after the coding is complete (e.g. player speed)

use amethyst::config::Config;
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use amethyst::utils::application_root_dir;

/// "Constants" that control the game mechanics
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GameConfiguration {
    /// bullet vertical velocity
    #[serde(default)]
    pub bullet_velocity: f32,
    /// how long to wait after firing a bullet before can fire again
    #[serde(default)]
    pub trigger_reset_timeout: f32,
    // alien horizontal velocity
    #[serde(default)]
    pub alien_velocity: f32,
}

// Default values
pub const BULLET_VELOCITY: f32 = 10.;
pub const TRIGGER_RESET_TIMEOUT: f32 = 0.5;
pub const ALIEN_VELOCITY: f32 = 20.;

impl Default for GameConfiguration {
    fn default() -> Self {
        GameConfiguration {
            bullet_velocity: BULLET_VELOCITY,
            trigger_reset_timeout: TRIGGER_RESET_TIMEOUT,
            alien_velocity: ALIEN_VELOCITY,
        }
    }
}

lazy_static! {
    /// The actual values for the [game configuration](struct.GameConfiguration.html)</a>.
    ///
    /// The game configuration is automatically loaded on startup
    /// from the file "game_config.ron" in resources.
    ///
    /// It's a good pattern for managing the game configuration
    /// so that the game designer can change parameters and balance
    /// the game without having to recompile the code.
    ///
    /// This variable looks to the remaining code as if it were a set of constants.
    pub static ref GAME_CONFIGURATION: GameConfiguration = {
        let game_config_path = format!(
            "{}/config/game.ron",
            env!("CARGO_MANIFEST_DIR")
        );
        GameConfiguration::load(&game_config_path).unwrap()
    };

}

//! Game state resources

use bevy_rapier2d::rapier::math::Vector;

/// Holds the main game state
#[derive(Default)]
pub struct GameConfig {
    /// Added gravity bonus for characters
    pub character_gravity: Vector<f32>,
}

/// The game state
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    Game,
    GameOver,
}

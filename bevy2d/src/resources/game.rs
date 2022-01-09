//! Game state resources

use bevy_rapier2d::prelude::*;

/// Holds the main game state
#[derive(Default)]
pub struct GameConfig {
    /// Added gravity bonus for characters
    pub character_gravity: Vector<f32>,
}

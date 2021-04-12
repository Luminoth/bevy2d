//! Game state resources

use bevy_rapier2d::rapier::math::Vector;

/// Holds the main game state
#[derive(Default)]
pub struct GameConfig {
    /// Added gravity bonus for characters
    pub character_gravity: Vector<f32>,
}

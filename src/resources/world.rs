//! Game... world... ? resources

use bevy::prelude::*;
use derivative::Derivative;

/// World bounding box
pub struct WorldBounds2D {
    /// The bottom left of the world
    pub min: Vec2,

    /// The top right of the world
    pub max: Vec2,
}

impl WorldBounds2D {
    /// Returns the width of the world
    pub fn width(&self) -> f32 {
        self.max.x() - self.min.x()
    }

    /// Returns the height of the world
    pub fn height(&self) -> f32 {
        self.max.y() - self.min.y()
    }
}

/// Holds world "system" configuration things
#[derive(Derivative)]
#[derivative(Default)]
pub struct WorldConfig {
    #[derivative(Default(value = "9.81"))]
    pub gravity: f32,
}

//! Collider components

use bevy::prelude::*;

/// A physics collider
pub struct Collider {
    // TODO: transform
    /// Collider bounds
    pub bounds: Rect<f32>,
}

impl Collider {
    pub fn width(&self) -> f32 {
        self.bounds.right - self.bounds.left
    }

    pub fn height(&self) -> f32 {
        self.bounds.top - self.bounds.bottom
    }
}

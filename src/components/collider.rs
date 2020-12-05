//! Platform components

use bevy::prelude::*;

/// A physics collider
pub struct Collider {
    /// Collider bounds
    pub bounds: Rect<f32>,
}

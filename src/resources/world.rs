use bevy::prelude::*;

pub struct WorldBounds2D {
    pub min: Vec2,
    pub max: Vec2,
}

impl WorldBounds2D {
    pub fn width(&self) -> f32 {
        self.max.x() - self.min.x()
    }

    pub fn height(&self) -> f32 {
        self.max.y() - self.min.y()
    }
}

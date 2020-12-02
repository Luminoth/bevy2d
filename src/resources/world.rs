use bevy::prelude::*;
use derivative::Derivative;

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

#[derive(Derivative)]
#[derivative(Default)]
pub struct WorldConfig {
    #[derivative(Default(value = "9.81"))]
    pub gravity: f32,
}

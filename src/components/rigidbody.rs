//! Rigid body physics components

use bevy::prelude::*;
use derivative::Derivative;

/// Janky rigid body component for physicsy things
#[derive(Derivative)]
#[derivative(Default)]
pub struct RigidBody2D {
    /// Kinematic things don't use physics
    pub kinematic: bool,

    pub acceleration: Vec2,
    pub velocity: Vec2,

    #[derivative(Default(value = "Vec2::new(20.0, 60.0)"))]
    pub max_velocity: Vec2,
}

impl RigidBody2D {
    /// Applies gravity to the rigid body's acceleration
    pub fn apply_gravity(&mut self, gravity: f32) {
        *self.acceleration.y_mut() -= gravity;
    }

    /// Updates the rigidbody (applies acceleration to the velocity)
    pub fn update(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
    }

    /// Updates the transforms translation with this rigid body's velocity
    pub fn apply(&self, transform: &mut Transform, dt: f32) {
        transform.translation += (self.velocity * dt).extend(0.0);

        // TODO: apply maximums
    }
}

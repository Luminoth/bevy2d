use bevy::prelude::*;

#[derive(Default)]
pub struct RigidBody2D {
    pub acceleration: Vec2,
    pub velocity: Vec2,
    pub kinematic: bool,
}

#[derive(Default)]
pub struct RigidBody {
    pub acceleration: Vec3,
    pub velocity: Vec3,
    pub kinematic: bool,
}

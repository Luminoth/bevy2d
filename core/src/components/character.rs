//! Character... ? components

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// A... character? I dunno
#[derive(Default, Component)]
pub struct Character {
    /// How fast the character moves (physics independent)
    pub speed: f32,

    /// What % of the character speed should be allowed for air control
    pub air_control_factor: f32,

    /// The character jump force
    pub jump_force: Vector<f32>,

    /// Is the character on the ground?
    pub grounded: bool,
}

/// A... player character? sure
#[derive(Default, Component)]
pub struct PlayerCharacter {}

/// A... non-player character? probably
#[derive(Default, Component)]
pub struct NonPlayerCharacter {}

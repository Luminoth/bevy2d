//! Character... ? components

use bevy_rapier2d::rapier::math::Vector;

/// A... character? I dunno
#[derive(Default)]
pub struct Character {
    /// How fast the character moves (physics independent)
    pub speed: f32,

    /// The character jump force
    pub jump_force: Vector<f32>,

    /// Is the character on the ground?
    pub grounded: bool,
}

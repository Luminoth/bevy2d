//! Character... ? components

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

/// A... character? I dunno
#[derive(Default, Component, Inspectable)]
pub struct Character {
    /// How fast the character moves (physics independent)
    pub speed: f32,

    /// What % of the character speed should be allowed for air control
    pub air_control_factor: f32,

    /// The character jump force
    pub jump_force: Vec2,

    /// Is the character on the ground?
    pub grounded: bool,
}

/// A... player character? sure
#[derive(Default, Component, Inspectable)]
pub struct PlayerCharacter {}

/// A... non-player character? probably
#[derive(Default, Component, Inspectable)]
pub struct NonPlayerCharacter {}

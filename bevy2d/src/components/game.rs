//! Game state components

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

/// Timer text marker
#[derive(Default, Component, Inspectable)]
pub struct TimerText;

//! ECS components

pub mod camera;
pub mod character;
pub mod debug;

use bevy::prelude::*;

#[derive(Default, Bundle)]
pub struct EmptyBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

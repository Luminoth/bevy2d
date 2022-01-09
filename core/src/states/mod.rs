//! Game state utils

use bevy::prelude::*;

/// Despawn all entities
pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Despawn all entities except for cameras
pub fn teardown_except_cameras(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

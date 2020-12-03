//! Physics simulation systems

use bevy::prelude::*;

use crate::components::rigidbody::*;
use crate::resources::world::*;

/// Process physics for all RigidBody2D components
pub fn process_rigidbodies_2d(
    time: Res<Time>,
    world_config: Res<WorldConfig>,
    mut query: Query<(&mut RigidBody2D, &mut Transform)>,
) {
    for (mut rigidbody, mut transform) in query.iter_mut() {
        // kinematic bodies don't have physics
        if rigidbody.kinematic {
            continue;
        }

        rigidbody.apply_gravity(world_config.gravity);

        rigidbody.update(time.delta_seconds);

        rigidbody.apply(&mut transform, time.delta_seconds);
    }
}

use bevy::prelude::*;

use crate::components::rigidbody::*;
use crate::resources::world::*;

pub fn process_rigidbodies_2d(
    time: Res<Time>,
    world_config: Res<WorldConfig>,
    mut query: Query<(&RigidBody2D, &mut Transform)>,
) {
    for (rigidbody, mut transform) in query.iter_mut() {
        if rigidbody.kinematic {
            continue;
        }

        *transform.translation.y_mut() -= world_config.gravity * time.delta_seconds;
    }
}

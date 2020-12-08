//! Character systems

use bevy::prelude::*;
use bevy_rapier2d::physics::RigidBodyHandleComponent;
use bevy_rapier2d::rapier::dynamics::RigidBodySet;

use crate::components::character::*;
use crate::resources::world::*;

/// Handles keyboard input for all characters
// TODO: this should only apply to *player* characters
// TODO: this broke when rapier was added... I don't think the transform can be used with a rigidbody
pub fn character_input_2d_keyboard_system(
    time: Res<Time>,
    world_bounds: Res<WorldBounds2D>,
    mut rigidbodies: ResMut<RigidBodySet>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Character, &Sprite, &RigidBodyHandleComponent)>,
) {
    let mut direction = Vec2::default();
    if keyboard_input.pressed(KeyCode::Right) {
        *direction.x_mut() += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        *direction.x_mut() -= 1.0;
    }

    for (character, sprite, rbhandle) in query.iter_mut() {
        if let Some(rigidbody) = rigidbodies.get_mut(rbhandle.handle()) {
            let half_width = sprite.size.x() / 2.0;

            let mut position = rigidbody.position().clone();

            let x = (position.translation.x + time.delta_seconds * direction.x() * character.speed)
                .min(world_bounds.max.x() - half_width)
                .max(world_bounds.min.x() + half_width);
            position.translation.x = x;

            rigidbody.set_next_kinematic_position(position)
        }
    }
}

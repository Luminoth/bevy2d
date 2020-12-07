//! Character systems

use bevy::prelude::*;

use crate::components::character::*;
use crate::resources::world::*;

/// Handles keyboard input for all characters
// TODO: this should only apply to *player* characters
pub fn character_input_2d_keyboard_system(
    time: Res<Time>,
    world_bounds: Res<WorldBounds2D>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Character, &Sprite, &mut Transform)>,
) {
    let mut direction = Vec2::default();
    if keyboard_input.pressed(KeyCode::Right) {
        *direction.x_mut() += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        *direction.x_mut() -= 1.0;
    }

    for (character, sprite, mut transform) in query.iter_mut() {
        let half_width = sprite.size.x() / 2.0;

        let x = (transform.translation.x() + time.delta_seconds * direction.x() * character.speed)
            .min(world_bounds.max.x() - half_width)
            .max(world_bounds.min.x() + half_width);
        transform.translation.set_x(x);
    }
}

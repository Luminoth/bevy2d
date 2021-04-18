//! Input systems

use bevy::prelude::*;

use crate::resources::input::CharacterInput2D;

/// 2D platformer keyboard input
pub fn platformer_2d_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut character_input: ResMut<CharacterInput2D>,
) {
    character_input.direction = Vec2::default();

    if keyboard_input.pressed(KeyCode::Right) {
        character_input.direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        character_input.direction.x -= 1.0;
    }
}

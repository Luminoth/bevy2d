//! Character systems

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use core_lib::components::character::*;
use core_lib::resources::input::*;

use crate::events::character::*;
use crate::resources::game::*;
use crate::resources::world::*;
use crate::{CHARACTER_LAYER, WORLD_LAYER};

/// Handles player character movement
pub fn character_movement(
    input: Res<CharacterInput2D>,
    bounds: Res<WorldBounds2D>,
    mut query: Query<(&Character, &Sprite, &GlobalTransform, &mut Velocity), With<PlayerCharacter>>,
) {
    for (character, sprite, transform, mut velocity) in query.iter_mut() {
        let half_width = sprite.custom_size.unwrap().x / 2.0;
        let position = transform.translation();
        let mut speed = character.speed;

        if (position.x - half_width < bounds.min.x && input.direction.x < 0.0)
            || (position.x + half_width > bounds.max.x && input.direction.x > 0.0)
        {
            // TODO: actually what we want is to wrap around
            speed = 0.0;
        } else if !character.grounded {
            // TODO: air control is kind of bad because we aren't factoring in momentum
            speed *= character.air_control_factor;
        }

        velocity.linvel.x = input.direction.x * speed;
    }
}

/// Handles player character jump events
pub fn character_jump(
    event_reader: EventReader<JumpEvent>,
    mut query: Query<(&Character, &mut ExternalImpulse), With<PlayerCharacter>>,
) {
    if !event_reader.is_empty() {
        event_reader.clear();

        for (character, mut impulse) in query.iter_mut() {
            if character.grounded {
                impulse.impulse = character.jump_force;
            }
        }
    }
}

/// Handles jump input
pub fn jump_input(keyboard_input: Res<Input<KeyCode>>, mut jump_events: EventWriter<JumpEvent>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        jump_events.send(JumpEvent);
    }
}

/// Characters fall faster for better mechanics
/// without having to affect the gravity effects of everything else
pub fn character_gravity_multiplier(
    game_config: Res<GameConfig>,
    mut query: Query<(&Character, &mut ExternalForce)>,
) {
    for (character, mut force) in query.iter_mut() {
        if !character.grounded {
            force.force += game_config.character_gravity;
        }
    }
}

/// Checks whether a character is on the ground or not
pub fn character_grounded_system(
    rapier_context: Res<RapierContext>,
    mut query: Query<(&mut Character, &Sprite, &Transform)>,
) {
    for (mut character, sprite, transform) in query.iter_mut() {
        let half_height = sprite.custom_size.unwrap().y / 2.0;

        let position = transform.translation;

        if let Some((_entity, _toi)) = rapier_context.cast_ray(
            Vec2::new(position.x, position.y - half_height),
            Vec2::Y * -1.0,
            0.1,
            true,
            QueryFilter::new().groups(InteractionGroups::new(CHARACTER_LAYER, WORLD_LAYER)),
        ) {
            if !character.grounded {
                debug!("grounded");
            }
            character.grounded = true;
        } else {
            if character.grounded {
                debug!("not grounded");
            }
            character.grounded = false;
        }
    }
}

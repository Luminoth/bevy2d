//! Character systems

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use core_lib::components::character::*;
use core_lib::resources::input::*;

use crate::events::character::*;
use crate::events::PauseEvent;
use crate::resources::game::*;
use crate::states::game::Game;
use crate::{CHARACTER_LAYER, WORLD_LAYER};

/// Handles player character movement
pub fn character_movement(
    game: Res<Game>,
    input: Res<CharacterInput2D>,
    mut query: Query<(&Character, &mut Velocity), With<PlayerCharacter>>,
) {
    if game.paused {
        return;
    }

    for (character, mut velocity) in query.iter_mut() {
        // TODO: air control is kind of bad because we aren't factoring in momentum
        let mut speed = character.speed;
        if !character.grounded {
            speed *= character.air_control_factor;
        }

        velocity.linvel.x = input.direction.x * speed;

        // TODO: need to set things up to constrain the character
        // or wrap it around the screen
    }
}

/// Handles player character jump events
pub fn character_jump(
    game: Res<Game>,
    mut event_reader: EventReader<JumpEvent>,
    mut query: Query<(&Character, &mut ExternalImpulse), With<PlayerCharacter>>,
) {
    if game.paused {
        return;
    }

    for _ in event_reader.iter() {
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

/// Handles pause events for characters
pub fn character_pause(
    mut commands: Commands,
    game: Res<Game>,
    mut event_reader: EventReader<PauseEvent>,
    query: Query<Entity, With<PlayerCharacter>>,
) {
    for _ in event_reader.iter() {
        if game.paused {
            for entity in query.iter() {
                commands
                    .entity(entity)
                    .remove::<RigidBody>()
                    .insert(RigidBody::Fixed);
            }
        } else {
            for entity in query.iter() {
                commands
                    .entity(entity)
                    .remove::<RigidBody>()
                    .insert(RigidBody::Dynamic);
            }
        }
    }
}

/// Characters fall faster for better mechanics
/// without having to affect the gravity effects of everything else
pub fn character_gravity_multiplier(
    game: Res<Game>,
    _game_config: Res<GameConfig>,
    mut query: Query<(&Character, &mut ExternalForce)>,
) {
    if game.paused {
        return;
    }

    for (character, mut _force) in query.iter_mut() {
        if !character.grounded {
            // TODO: this is waaaay too much
            //force.force += game_config.character_gravity;
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

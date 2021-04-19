//! Character systems

use bevy::prelude::*;
use bevy_rapier2d::physics::RigidBodyHandleComponent;
use bevy_rapier2d::rapier::dynamics::{BodyStatus, RigidBodySet};
use bevy_rapier2d::rapier::geometry::{ColliderSet, Ray};
use bevy_rapier2d::rapier::math::{Point, Vector};
use bevy_rapier2d::rapier::pipeline::QueryPipeline;

use core_lib::components::character::*;
use core_lib::resources::input::*;

use crate::events::character::*;
use crate::events::PauseEvent;
use crate::resources::game::*;
use crate::resources::world::*;
use crate::states::game::Game;
use crate::CHARACTER_COLLISION_GROUPS;

/// Handles player character movement
pub fn character_movement(
    time: Res<Time>,
    world_bounds: Res<WorldBounds2D>,
    game: Res<Game>,
    mut rigidbodies: ResMut<RigidBodySet>,
    input: Res<CharacterInput2D>,
    mut query: Query<(&Character, &Sprite, &RigidBodyHandleComponent), With<PlayerCharacter>>,
) {
    if game.paused {
        return;
    }

    for (character, sprite, rbhandle) in query.iter_mut() {
        if let Some(rigidbody) = rigidbodies.get_mut(rbhandle.handle()) {
            // TODO: air control is kind of bad because we aren't factoring in momentum
            let mut speed = character.speed;
            if !character.grounded {
                speed *= character.air_control_factor;
            }

            let half_width = sprite.size.x / 2.0;

            let mut position = *rigidbody.position();

            let x = (position.translation.x
                + time.delta().as_secs_f32() * input.direction.x * speed)
                .min(world_bounds.max.x - half_width)
                .max(world_bounds.min.x + half_width);
            position.translation.x = x;

            rigidbody.set_position(position, false);
        }
    }
}

/// Handles player character jump events
pub fn character_jump(
    game: Res<Game>,
    mut rigidbodies: ResMut<RigidBodySet>,
    mut event_reader: EventReader<JumpEvent>,
    mut query: Query<(&Character, &RigidBodyHandleComponent), With<PlayerCharacter>>,
) {
    if game.paused {
        return;
    }

    for _ in event_reader.iter() {
        for (character, rbhandle) in query.iter_mut() {
            if let Some(rigidbody) = rigidbodies.get_mut(rbhandle.handle()) {
                if character.grounded {
                    rigidbody.apply_impulse(character.jump_force, true)
                }
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
    game: Res<Game>,
    mut rigidbodies: ResMut<RigidBodySet>,
    mut event_reader: EventReader<PauseEvent>,
    mut query: Query<&RigidBodyHandleComponent, With<PlayerCharacter>>,
) {
    for _ in event_reader.iter() {
        for rbhandle in query.iter_mut() {
            if let Some(rigidbody) = rigidbodies.get_mut(rbhandle.handle()) {
                rigidbody.set_body_status(if game.paused {
                    BodyStatus::Kinematic
                } else {
                    BodyStatus::Dynamic
                })
            }
        }
    }
}

/// Characters fall faster for better mechanics
/// without having to affect the gravity effects of everything else
pub fn character_gravity_multiplier(
    game: Res<Game>,
    game_config: Res<GameConfig>,
    mut rigidbodies: ResMut<RigidBodySet>,
    mut query: Query<(&Character, &RigidBodyHandleComponent)>,
) {
    if game.paused {
        return;
    }

    for (character, rbhandle) in query.iter_mut() {
        if let Some(rigidbody) = rigidbodies.get_mut(rbhandle.handle()) {
            if !character.grounded {
                rigidbody.apply_force(game_config.character_gravity, true);
            }
        }
    }
}

/// Checks whether a character is on the ground or not
pub fn character_grounded_system(
    qp: Res<QueryPipeline>,
    mut rigidbodies: ResMut<RigidBodySet>,
    colliders: Res<ColliderSet>,
    mut query: Query<(&mut Character, &Sprite, &RigidBodyHandleComponent)>,
) {
    for (mut character, sprite, rbhandle) in query.iter_mut() {
        if let Some(rigidbody) = rigidbodies.get_mut(rbhandle.handle()) {
            let half_height = sprite.size.y / 2.0;

            let position = rigidbody.position();

            let ray = Ray::new(
                Point::new(position.translation.x, position.translation.y - half_height),
                Vector::y() * -1.0,
            );

            if let Some((_handle, _collider)) = qp.cast_ray(
                &colliders,
                &ray,
                0.1,
                true,
                *CHARACTER_COLLISION_GROUPS,
                None,
            ) {
                character.grounded = true;
            } else {
                character.grounded = false;
            }
        }
    }
}

//! Character systems

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
    input: Res<CharacterInput2D>,
    mut query: Query<(&Character, &Sprite, &mut RigidBodyPositionComponent), With<PlayerCharacter>>,
) {
    if game.paused {
        return;
    }

    for (character, sprite, mut rb_pos) in query.iter_mut() {
        // TODO: air control is kind of bad because we aren't factoring in momentum
        let mut speed = character.speed;
        if !character.grounded {
            speed *= character.air_control_factor;
        }

        let half_width = sprite.custom_size.unwrap().x / 2.0;

        let mut position = rb_pos.position;

        let x = (position.translation.x + time.delta().as_secs_f32() * input.direction.x * speed)
            .min(world_bounds.max.x - half_width)
            .max(world_bounds.min.x + half_width);
        position.translation.x = x;

        rb_pos.position = position;
    }
}

/// Handles player character jump events
pub fn character_jump(
    game: Res<Game>,
    mut event_reader: EventReader<JumpEvent>,
    mut query: Query<
        (
            &Character,
            &mut RigidBodyVelocityComponent,
            &RigidBodyMassPropsComponent,
        ),
        With<PlayerCharacter>,
    >,
) {
    if game.paused {
        return;
    }

    for _ in event_reader.iter() {
        for (character, mut rb_vel, rb_mass) in query.iter_mut() {
            if character.grounded {
                rb_vel.apply_impulse(rb_mass, character.jump_force)
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
    mut event_reader: EventReader<PauseEvent>,
    mut query: Query<&mut RigidBodyTypeComponent, With<PlayerCharacter>>,
) {
    for _ in event_reader.iter() {
        for mut rb_type in query.iter_mut() {
            rb_type.0 = if game.paused {
                RigidBodyType::Static
            } else {
                //RigidBodyType::KinematicPositionBased
                RigidBodyType::Dynamic
            };
        }
    }
}

/// Characters fall faster for better mechanics
/// without having to affect the gravity effects of everything else
pub fn character_gravity_multiplier(
    game: Res<Game>,
    game_config: Res<GameConfig>,
    mut query: Query<(&Character, &mut RigidBodyForcesComponent)>,
) {
    if game.paused {
        return;
    }

    for (character, mut rb_force) in query.iter_mut() {
        if !character.grounded {
            rb_force.force += game_config.character_gravity;
        }
    }
}

/// Checks whether a character is on the ground or not
pub fn character_grounded_system(
    qp: Res<QueryPipeline>,
    cq: QueryPipelineColliderComponentsQuery,
    mut query: Query<(&mut Character, &Sprite, &RigidBodyPositionComponent)>,
) {
    let colliders = QueryPipelineColliderComponentsSet(&cq);

    for (mut character, sprite, rb_pos) in query.iter_mut() {
        let half_height = sprite.custom_size.unwrap().y / 2.0;

        let position = rb_pos.position;

        let ray = Ray::new(
            Vec2::new(position.translation.x, position.translation.y - half_height).into(),
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

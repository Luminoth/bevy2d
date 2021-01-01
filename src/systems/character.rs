//! Character systems

use bevy::prelude::*;
use bevy_rapier2d::physics::RigidBodyHandleComponent;
use bevy_rapier2d::rapier::dynamics::RigidBodySet;
use bevy_rapier2d::rapier::geometry::{ColliderSet, Ray};
use bevy_rapier2d::rapier::math::{Point, Vector};
use bevy_rapier2d::rapier::pipeline::QueryPipeline;

use crate::components::character::*;
use crate::resources::game::*;
use crate::resources::world::*;
use crate::CHARACTER_COLLISION_GROUPS;

/// Handles keyboard input for all kinematic characters
// TODO: this should only apply to *player* characters
pub fn character_input_2d_keyboard_system(
    time: Res<Time>,
    world_bounds: Res<WorldBounds2D>,
    mut rigidbodies: ResMut<RigidBodySet>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Character, &Sprite, &RigidBodyHandleComponent)>,
) {
    let mut direction = Vec2::default();
    if keyboard_input.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }

    for (character, sprite, rbhandle) in query.iter_mut() {
        if let Some(rigidbody) = rigidbodies.get_mut(rbhandle.handle()) {
            /*if !character.grounded {
                continue;
            }*/

            let half_width = sprite.size.x / 2.0;

            let mut position = *rigidbody.position();

            let x = (position.translation.x + time.delta_seconds() * direction.x * character.speed)
                .min(world_bounds.max.x - half_width)
                .max(world_bounds.min.x + half_width);
            position.translation.x = x;

            rigidbody.set_position(position, false);

            if character.grounded && keyboard_input.just_pressed(KeyCode::Space) {
                println!("jump");
                rigidbody.apply_impulse(character.jump_force, false)
            }
        }
    }
}

/// Characters fall faster for better mechanics
/// without having to affect the gravity effects of everything else
pub fn character_gravity_multiplier(
    game_config: Res<GameConfig>,
    mut rigidbodies: ResMut<RigidBodySet>,
    mut query: Query<(&Character, &RigidBodyHandleComponent)>,
) {
    for (character, rbhandle) in query.iter_mut() {
        if let Some(rigidbody) = rigidbodies.get_mut(rbhandle.handle()) {
            if !character.grounded {
                rigidbody.apply_force(game_config.character_gravity, false);
                //rigidbody.apply_impulse(game_config.character_gravity, false);
            }
        }
    }
}

/// Checks whether a character is on the ground or not
pub fn character_grounded_systems(
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

            let grounded = character.grounded;
            if let Some((_handle, _collider, _intersection)) =
                qp.cast_ray(&colliders, &ray, 0.1, *CHARACTER_COLLISION_GROUPS)
            {
                character.grounded = true;
                if character.grounded != grounded {
                    println!("setting grounded");
                }

            // adjust position so we don't fall through the collision
            /*let mut position = *rigidbody.position();
            let point = ray.point_at(intersection.toi);
            position.translation.y = point.coords.y + half_height + 0.05;

            rigidbody.set_position(position, false);*/
            } else {
                character.grounded = false;
                if character.grounded != grounded {
                    println!("not grounded");
                }
            }
        }
    }
}

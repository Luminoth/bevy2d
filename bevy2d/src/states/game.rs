//! Game state systems

use bevy::prelude::*;
use bevy_rapier2d::physics::RapierConfiguration;
use bevy_rapier2d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier2d::rapier::geometry::ColliderBuilder;
use bevy_rapier2d::rapier::math::Vector;

use core_lib::components::camera::*;
use core_lib::components::character::*;
use core_lib::resources::input::*;

use crate::resources::game::*;
use crate::resources::world::*;
use crate::{
    ASPECT_RATIO, CAMERA_SIZE, CHARACTER_COLLISION_GROUPS, CHARACTER_GRAVITY, CHARACTER_JUMP_FORCE,
    CHARACTER_MASS, GRAVITY, WORLD_COLLISION_GROUPS,
};

#[derive(Default)]
pub struct GameEntities {
    camera: Option<Entity>,
}

/// Game setup
pub fn setup(mut commands: Commands) {
    info!("camera size: {}", CAMERA_SIZE);

    let mut entities = GameEntities::default();

    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    entities.camera = Some(
        commands
            //spawn_bundle(OrthographicCameraBundle::new_2d());
            .spawn_bundle(CameraOrtho2dBundle::new(CAMERA_SIZE))
            //spawn_bundle(CameraOrtho2dBundle::new_2d());
            .id(),
    );

    // physics
    commands.insert_resource(RapierConfiguration {
        gravity: Vector::y() * GRAVITY,
        ..Default::default()
    });

    // game state
    commands.insert_resource(CharacterInput2D::default());
    commands.insert_resource(GameConfig {
        character_gravity: Vector::y() * CHARACTER_GRAVITY,
    });

    commands.insert_resource(entities);
}

/// Game teardown
pub fn teardown(mut commands: Commands, mut entities: ResMut<GameEntities>) {
    if let Some(camera) = entities.camera.take() {
        commands.entity(camera).despawn_recursive();
    }

    commands.remove_resource::<GameEntities>();

    commands.remove_resource::<CharacterInput2D>();
    commands.remove_resource::<GameConfig>();
    commands.remove_resource::<RapierConfiguration>();
    commands.remove_resource::<ClearColor>();
}

#[derive(Default)]
pub struct GameWorldEntities {
    entities: Vec<Entity>,
}

/// Setup the game world
pub fn setup_world(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let world_bounds = WorldBounds2D {
        min: Vec2::new(-ASPECT_RATIO * CAMERA_SIZE, -CAMERA_SIZE),
        max: Vec2::new(ASPECT_RATIO * CAMERA_SIZE, CAMERA_SIZE),
    };

    // world
    commands.insert_resource(world_bounds);

    let mut entities = GameWorldEntities::default();

    // ground
    entities.entities.push(
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                sprite: Sprite::new(Vec2::new(world_bounds.width(), 1.0)),
                ..Default::default()
            })
            .insert(RigidBodyBuilder::new_static().translation(0.0, world_bounds.min.y + 0.5))
            .insert(
                ColliderBuilder::cuboid(world_bounds.width() / 2.0, 0.5)
                    .collision_groups(*WORLD_COLLISION_GROUPS)
                    .friction(0.0)
                    .restitution(0.0),
            )
            .id(),
    );

    // platforms
    entities.entities.push(
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
                sprite: Sprite::new(Vec2::new(5.0, 1.0)),
                ..Default::default()
            })
            .insert(RigidBodyBuilder::new_static().translation(0.0, 0.0))
            .insert(
                ColliderBuilder::cuboid(2.5, 0.5)
                    .collision_groups(*WORLD_COLLISION_GROUPS)
                    .friction(0.0)
                    .restitution(0.0),
            )
            .id(),
    );

    entities.entities.push(
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(0.0, 1.0, 1.0).into()),
                sprite: Sprite::new(Vec2::new(5.0, 1.0)),
                ..Default::default()
            })
            .insert(RigidBodyBuilder::new_static().translation(-10.0, -5.0))
            .insert(
                ColliderBuilder::cuboid(2.5, 0.5)
                    .collision_groups(*WORLD_COLLISION_GROUPS)
                    .friction(0.0)
                    .restitution(0.0),
            )
            .id(),
    );

    entities.entities.push(
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(0.0, 1.0, 1.0).into()),
                sprite: Sprite::new(Vec2::new(5.0, 1.0)),
                ..Default::default()
            })
            .insert(RigidBodyBuilder::new_static().translation(10.0, -5.0))
            .insert(
                ColliderBuilder::cuboid(2.5, 0.5)
                    .collision_groups(*WORLD_COLLISION_GROUPS)
                    .friction(0.0)
                    .restitution(0.0),
            )
            .id(),
    );

    // characters
    entities.entities.push(
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
                sprite: Sprite::new(Vec2::new(1.0, 2.0)),
                ..Default::default()
            })
            .insert(
                //RigidBodyBuilder::new_kinematic()
                RigidBodyBuilder::new_dynamic()
                    .translation(world_bounds.min.x + 1.0, world_bounds.min.y + 2.0)
                    .additional_mass(CHARACTER_MASS)
                    .lock_rotations(),
            )
            .insert(
                ColliderBuilder::cuboid(0.5, 1.0)
                    .collision_groups(*CHARACTER_COLLISION_GROUPS)
                    .friction(0.0)
                    .restitution(0.0),
            )
            .insert(Character {
                speed: 10.0,
                air_control_factor: 1.0,
                jump_force: Vector::y() * CHARACTER_JUMP_FORCE,
                ..Default::default()
            })
            .insert(PlayerCharacter::default())
            .id(),
    );

    commands.insert_resource(entities);
}

/// Tear down the game world
pub fn teardown_world(mut commands: Commands, mut entities: ResMut<GameWorldEntities>) {
    for entity in entities.entities.drain(..) {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<GameWorldEntities>();

    commands.remove_resource::<WorldBounds2D>();
}

#[derive(Default)]
pub struct GameUIEntities {
    camera: Option<Entity>,
}

/// Setup the game UI
pub fn setup_ui(mut commands: Commands) {
    let mut entities = GameUIEntities::default();

    entities.camera = Some(commands.spawn_bundle(UiCameraBundle::default()).id());

    commands.insert_resource(entities);
}

/// Tear down the game UI
pub fn teardown_ui(mut commands: Commands, mut entities: ResMut<GameUIEntities>) {
    if let Some(camera) = entities.camera.take() {
        commands.entity(camera).despawn_recursive();
    }

    commands.remove_resource::<GameUIEntities>();
}

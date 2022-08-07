//! Game state systems

use bevy::prelude::*;
use bevy::render::camera::*;
use bevy_rapier2d::prelude::*;

use core_lib::components::character::*;
use core_lib::resources::input::*;

use crate::components::game::*;
use crate::resources::game::*;
use crate::resources::world::*;
use crate::states::*;
use crate::{
    ASPECT_RATIO, CAMERA_SIZE, CHARACTER_GRAVITY, CHARACTER_JUMP_FORCE, CHARACTER_LAYER,
    CHARACTER_MASS, GRAVITY, WORLD_LAYER,
};

/// Main game state
#[derive(Default)]
pub struct Game {
    timer: Timer,

    pub paused: bool,
}

/// Game setup
pub fn setup(mut commands: Commands) {
    info!("camera size: {}", CAMERA_SIZE);

    // cameras
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(CAMERA_SIZE * 2.0);

    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands.spawn_bundle(camera);

    // physics
    commands.insert_resource(RapierConfiguration {
        gravity: Vec2::Y * GRAVITY,
        ..Default::default()
    });

    // game state
    commands.insert_resource(CharacterInput2D::default());
    commands.insert_resource(GameConfig {
        character_gravity: Vec2::Y * CHARACTER_GRAVITY,
    });
    commands.insert_resource(Game {
        timer: Timer::from_seconds(30.0, false),
        ..Default::default()
    });
}

/// Game teardown
pub fn teardown(mut commands: Commands) {
    commands.remove_resource::<Game>();
    commands.remove_resource::<GameConfig>();
    commands.remove_resource::<CharacterInput2D>();
    commands.remove_resource::<RapierConfiguration>();
    commands.remove_resource::<ClearColor>();
}

/// Setup the game world
pub fn setup_world(mut commands: Commands) {
    let world_bounds = WorldBounds2D {
        min: Vec2::new(-ASPECT_RATIO * CAMERA_SIZE, -CAMERA_SIZE),
        max: Vec2::new(ASPECT_RATIO * CAMERA_SIZE, CAMERA_SIZE),
    };

    // world
    commands.insert_resource(world_bounds);

    // ground
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(world_bounds.width(), 1.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, world_bounds.min.y + 0.5, 0.0)),
            ..Default::default()
        })
        // rigidbody
        .insert(RigidBody::Fixed)
        .insert(LockedAxes::ROTATION_LOCKED)
        //.insert(RigidBodyPositionSync::Discrete)
        // collider
        .insert(Collider::cuboid(world_bounds.width() / 2.0, 0.5))
        //.insert(ColliderMaterial::new(0.0, 0.0))
        .insert(CollisionGroups::new(WORLD_LAYER, CHARACTER_LAYER))
        .insert(Name::new("Ground"));

    // platforms
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(Vec2::new(5.0, 1.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        // rigidbody
        .insert(RigidBody::Fixed)
        .insert(LockedAxes::ROTATION_LOCKED)
        //.insert(RigidBodyPositionSync::Discrete)
        // collider
        .insert(Collider::cuboid(2.5, 0.5))
        //.insert(ColliderMaterial::new(0.0, 0.0))
        .insert(CollisionGroups::new(WORLD_LAYER, CHARACTER_LAYER))
        .insert(Name::new("Platform"));

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::CYAN,
                custom_size: Some(Vec2::new(5.0, 1.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(-10.0, -5.0, 0.0)),
            ..Default::default()
        })
        // rigidbody
        .insert(RigidBody::Fixed)
        .insert(LockedAxes::ROTATION_LOCKED)
        //.insert(RigidBodyPositionSync::Discrete)
        // collider
        .insert(Collider::cuboid(2.5, 0.5))
        //.insert(ColliderMaterial::new(0.0, 0.0))
        .insert(CollisionGroups::new(WORLD_LAYER, CHARACTER_LAYER))
        .insert(Name::new("Platform"));

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::CYAN,
                custom_size: Some(Vec2::new(5.0, 1.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(10.0, -5.0, 0.0)),
            ..Default::default()
        })
        // rigidbody
        .insert(RigidBody::Fixed)
        .insert(LockedAxes::ROTATION_LOCKED)
        //.insert(RigidBodyPositionSync::Discrete)
        // collider
        .insert(Collider::cuboid(2.5, 0.5))
        //.insert(ColliderMaterial::new(0.0, 0.0))
        .insert(CollisionGroups::new(WORLD_LAYER, CHARACTER_LAYER))
        .insert(Name::new("Platform"));

    // player
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::new(1.0, 2.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                world_bounds.min.x + 1.0,
                world_bounds.min.y + 10.0,
                0.0,
            )),
            ..Default::default()
        })
        // rigidbody
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(AdditionalMassProperties::MassProperties(MassProperties {
            local_center_of_mass: Vec2::ZERO,
            mass: CHARACTER_MASS,
            principal_inertia: 0.0,
        }))
        .insert(Velocity::default())
        //.insert(RigidBodyPositionSync::Discrete)
        // collider
        .insert(Collider::cuboid(0.5, 1.0))
        //.insert(ColliderMaterial::new(0.0, 0.0))
        .insert(CollisionGroups::new(CHARACTER_LAYER, WORLD_LAYER))
        .insert(Character {
            speed: 10.0,
            air_control_factor: 1.0,
            jump_force: Vec2::Y * CHARACTER_JUMP_FORCE,
            ..Default::default()
        })
        // forces
        .insert(ExternalForce::default())
        .insert(ExternalImpulse::default())
        .insert(PlayerCharacter::default())
        .insert(Name::new("Player"));
}

/// Tear down the game world
pub fn teardown_world(mut commands: Commands) {
    commands.remove_resource::<WorldBounds2D>();
}

/// Setup the game UI
pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::from_section(
                "0",
                TextStyle {
                    font: asset_server.load("fonts/Roboto-Regular.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            ..Default::default()
        })
        .insert(TimerText);
}

/// Tear down the game UI
pub fn teardown_ui(mut _commands: Commands) {}

/// Main game update
pub fn on_update(time: Res<Time>, mut game: ResMut<Game>, mut state: ResMut<State<GameState>>) {
    if game.paused {
        return;
    }

    if game.timer.tick(time.delta()).just_finished() {
        state.push(GameState::GameOver).unwrap();
    }
}

/// Game UI update
pub fn update_ui(game: Res<Game>, mut query: Query<&mut Text, With<TimerText>>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!(
            "{}",
            (game.timer.duration().as_secs_f32() - game.timer.elapsed_secs()) as i64
        );
    }
}

//! Herpa derp but this time in TWO dimensions!

mod components;
mod events;
mod resources;
mod systems;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::physics::{RapierConfiguration, RapierPhysicsPlugin};
use bevy_rapier2d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier2d::rapier::geometry::{ColliderBuilder, InteractionGroups};
use bevy_rapier2d::rapier::math::Vector;
use once_cell::sync::Lazy;

use components::camera::*;
use components::character::*;
use events::debug::*;
use resources::debug::*;
use resources::game::*;
use resources::world::*;
use systems::character::*;
use systems::debug::*;
use systems::world::*;

// physics layers
const WORLD_LAYER: u16 = 0b01;
const CHARACTER_LAYER: u16 = 0b10;

// physics collision groups
static WORLD_COLLISION_GROUPS: Lazy<InteractionGroups> =
    Lazy::new(|| InteractionGroups::new(WORLD_LAYER, CHARACTER_LAYER));
static CHARACTER_COLLISION_GROUPS: Lazy<InteractionGroups> =
    Lazy::new(|| InteractionGroups::new(CHARACTER_LAYER, WORLD_LAYER));

const GRAVITY: f32 = -9.81;

const CHARACTER_MASS: f32 = 100.0;
const CHARACTER_GRAVITY: f32 = -750.0;
const CHARACTER_JUMP_FORCE: f32 = 1500.0;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const ASPECT_RATIO: f32 = WINDOW_WIDTH / WINDOW_HEIGHT;

// https://indiehoodgames.wordpress.com/2013/07/27/pixel-perfect-calculator-for-orthographic-camera-unity3d/
const PIXELS_PER_UNIT: f32 = 32.0;
const CAMERA_SIZE: f32 = WINDOW_HEIGHT / (2.0 * PIXELS_PER_UNIT);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("camera size: {}", CAMERA_SIZE);

    #[cfg(debug_assertions)]
    asset_server.watch_for_changes().unwrap();

    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    //commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(CameraOrtho2dBundle::new(CAMERA_SIZE));
    //commands.spawn_bundle(CameraOrtho2dBundle::new_2d());

    // physics
    commands.insert_resource(RapierConfiguration {
        gravity: Vector::y() * GRAVITY,
        ..Default::default()
    });

    // game state
    commands.insert_resource(GameConfig {
        character_gravity: Vector::y() * CHARACTER_GRAVITY,
    });
    commands.insert_resource(State::new(GameState::Game));
}

fn setup_world(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let world_bounds = WorldBounds2D {
        min: Vec2::new(-ASPECT_RATIO * CAMERA_SIZE, -CAMERA_SIZE),
        max: Vec2::new(ASPECT_RATIO * CAMERA_SIZE, CAMERA_SIZE),
    };

    // world
    commands.insert_resource(world_bounds);

    // ground
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
        );

    // platforms
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
        );

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
        );

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
        );

    // characters
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
        });
}

fn setup_ui(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_debug(mut commands: Commands) {
    commands.insert_resource(DebugState::default());
}

#[bevy_main]
fn main() {
    // TODO: setup game state stages per https://bevyengine.org/news/bevy-0-4/
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Bevy 2D".to_owned(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin)
        //.add_plugin(bevy_rapier2d::render::RapierRenderPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_event::<ToggleDebugEvent>()
        .add_startup_system(setup.system())
        .add_startup_system(setup_world.system())
        .add_startup_system(setup_ui.system())
        .add_startup_system(setup_debug.system())
        // add internal camera system update
        .add_system_to_stage(
            bevy::app::CoreStage::PostUpdate,
            bevy::render::camera::camera_system::<OrthoProjection>.system(),
        )
        // input
        .add_system(character_input_2d_keyboard_system.system())
        // physics
        .add_system(character_grounded_system.system())
        .add_system(character_gravity_multiplier.system())
        // debug
        .add_system(debug_system.system())
        .add_system(world_bounds_toggle_debug_system.system())
        .add_system(fps_text_system.system())
        .run();
}

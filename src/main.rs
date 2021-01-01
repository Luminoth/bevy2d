//! Herpa derp but this time in TWO dimensions!

mod components;
mod events;
mod resources;
mod systems;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
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

const WORLD_LAYER: u16 = 0b01;
const CHARACTER_LAYER: u16 = 0b10;

static WORLD_COLLISION_GROUPS: Lazy<InteractionGroups> =
    Lazy::new(|| InteractionGroups::new(WORLD_LAYER, CHARACTER_LAYER));
static CHARACTER_COLLISION_GROUPS: Lazy<InteractionGroups> =
    Lazy::new(|| InteractionGroups::new(CHARACTER_LAYER, WORLD_LAYER));

const GRAVITY: f32 = -9.81;

const CHARACTER_MASS: f32 = 70.0;
const CHARACTER_GRAVITY: f32 = -750.0;
const CHARACTER_JUMP_FORCE: f32 = 500.0;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const ASPECT_RATIO: f32 = WINDOW_WIDTH / WINDOW_HEIGHT;

// https://indiehoodgames.wordpress.com/2013/07/27/pixel-perfect-calculator-for-orthographic-camera-unity3d/
const PIXELS_PER_UNIT: f32 = 32.0;
const CAMERA_SIZE: f32 = WINDOW_HEIGHT / (2.0 * PIXELS_PER_UNIT);

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    println!("camera size: {}", CAMERA_SIZE);

    #[cfg(debug_assertions)]
    asset_server.watch_for_changes().unwrap();

    commands
        // cameras
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .spawn(CameraOrtho2dBundle::new(CAMERA_SIZE))
        // physics
        .insert_resource(RapierConfiguration {
            gravity: Vector::y() * GRAVITY,
            ..Default::default()
        })
        // game state
        .insert_resource(GameConfig {
            character_gravity: Vector::y() * CHARACTER_GRAVITY,
        })
        .insert_resource(GameState::default());
}

fn setup_world(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // world
    commands
        // TODO: we need a component to update this whenever the window size changes
        .insert_resource(WorldBounds2D {
            min: Vec2::new(-ASPECT_RATIO * CAMERA_SIZE, -CAMERA_SIZE),
            max: Vec2::new(ASPECT_RATIO * CAMERA_SIZE, CAMERA_SIZE),
        });

    // TODO: spawning just a single floor that spans the entire width would be better
    for x in 0..41 {
        commands
            .spawn(SpriteBundle {
                material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                sprite: Sprite::new(Vec2::new(1.0, 1.0)),
                ..Default::default()
            })
            .with(
                RigidBodyBuilder::new_static()
                    .translation((-ASPECT_RATIO * CAMERA_SIZE) + x as f32, -CAMERA_SIZE + 0.5),
            )
            .with(
                ColliderBuilder::cuboid(0.5, 0.5)
                    .collision_groups(*WORLD_COLLISION_GROUPS)
                    .friction(0.0)
                    .restitution(0.0),
            );
    }

    // characters
    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
            sprite: Sprite::new(Vec2::new(1.0, 2.0)),
            ..Default::default()
        })
        .with(Character {
            speed: 10.0,
            jump_force: Vector::y() * CHARACTER_JUMP_FORCE,
            ..Default::default()
        })
        .with(
            //RigidBodyBuilder::new_kinematic()
            RigidBodyBuilder::new_dynamic()
                .translation(0.0, 0.0)
                .mass(CHARACTER_MASS, false)
                .lock_rotations(),
        )
        .with(
            ColliderBuilder::cuboid(0.5, 1.0)
                .collision_groups(*CHARACTER_COLLISION_GROUPS)
                .friction(0.0)
                .restitution(0.0),
        );
}

fn setup_ui(commands: &mut Commands) {
    commands.spawn(CameraUiBundle::default());
}

fn setup_debug(commands: &mut Commands) {
    commands.insert_resource(DebugState::default());
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Bevy 2D".to_owned(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
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
            bevy::app::stage::POST_UPDATE,
            bevy::render::camera::camera_system::<OrthoProjection>.system(),
        )
        // input
        .add_system(character_input_2d_keyboard_system.system())
        // physics
        .add_system(character_grounded_systems.system())
        .add_system(character_gravity_multiplier.system())
        // debug
        .add_system(debug_system.system())
        .add_system(world_bounds_toggle_debug_system.system())
        .add_system(fps_text_system.system())
        .run();
}

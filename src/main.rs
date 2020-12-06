//! Herpa derp but this time in TWO dimensions!

mod components;
mod resources;
mod systems;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

use components::camera::*;
use components::character::*;
use components::collider::*;
use components::rigidbody::*;
use resources::game::*;
use resources::world::*;
use systems::character::*;
use systems::debug::*;
use systems::physics::*;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const ASPECT_RATIO: f32 = WINDOW_WIDTH / WINDOW_HEIGHT;

// https://indiehoodgames.wordpress.com/2013/07/27/pixel-perfect-calculator-for-orthographic-camera-unity3d/
const PIXELS_PER_UNIT: f32 = 32.0;
const CAMERA_SIZE: f32 = WINDOW_HEIGHT / (2.0 * PIXELS_PER_UNIT);

fn setup(asset_server: Res<AssetServer>) {
    #[cfg(debug_assertions)]
    asset_server.watch_for_changes().unwrap();
}

fn setup_world(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    println!("camera size: {}", CAMERA_SIZE);

    commands
        .spawn(Ortho2dComponents::new(CAMERA_SIZE))
        // characters
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
            sprite: Sprite::new(Vec2::new(1.0, 2.0)),
            ..Default::default()
        })
        .with(Character { speed: 10.0 })
        .with(RigidBody2D {
            kinematic: true,
            ..Default::default()
        });

    // world
    // TODO: spawning just a single floor that spans the entire width would be better
    for x in 0..41 {
        commands
            .spawn(SpriteComponents {
                material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                sprite: Sprite::new(Vec2::new(1.0, 1.0)),
                // TODO: we need a component to update this
                // whenever the window size changes
                transform: Transform::from_translation(Vec3::new(
                    (-ASPECT_RATIO * CAMERA_SIZE) + x as f32,
                    -CAMERA_SIZE + 0.5,
                    0.0,
                )),
                ..Default::default()
            })
            .with(Collider {
                bounds: Rect {
                    left: 0.0,
                    right: 1.0,
                    bottom: 0.0,
                    top: 0.0,
                },
            });
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(UiCameraComponents::default());
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Bevy 2D".to_owned(),
            width: WINDOW_WIDTH as u32,
            height: WINDOW_HEIGHT as u32,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(GameState::default())
        // TODO: we need a component to update this
        // whenever the window size changes
        .add_resource(WorldBounds2D {
            min: Vec2::new(-ASPECT_RATIO * CAMERA_SIZE, -CAMERA_SIZE),
            max: Vec2::new(ASPECT_RATIO * CAMERA_SIZE, CAMERA_SIZE),
        })
        .add_resource(WorldConfig::default())
        .add_startup_system(setup.system())
        .add_startup_system(setup_world.system())
        .add_startup_system(setup_ui.system())
        // add internal camera system update
        .add_system_to_stage(
            bevy::app::stage::POST_UPDATE,
            bevy::render::camera::camera_system::<OrthoProjection>.system(),
        )
        .add_system(character_input_2d_keyboard_system.system())
        .add_system(check_grounded_system.system())
        .add_system(process_rigidbodies_2d_system.system())
        .add_system(process_collisions_system.system())
        .add_system(debug_system.system())
        .add_system(fps_text_system.system())
        .run();
}

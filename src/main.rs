//! Herpa derp but this time in TWO dimensions!

mod components;
mod resources;
mod systems;

use bevy::prelude::*;

use components::camera::*;
use components::character::*;
use components::rigidbody::*;
use resources::game::*;
use resources::world::*;
use systems::character::*;
use systems::debug::*;
use systems::physics::*;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const SPRITE_SIZE: f32 = 32.0;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // https://indiehoodgames.wordpress.com/2013/07/27/pixel-perfect-calculator-for-orthographic-camera-unity3d/
    let camera_size = WINDOW_HEIGHT / (2.0 * SPRITE_SIZE);
    println!("camera size: {}", camera_size);

    commands
        // cameras
        .spawn(Ortho2dComponents::new(camera_size))
        .spawn(UiCameraComponents::default())
        // characters
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
            sprite: Sprite::new(Vec2::new(1.0, 2.0)),
            ..Default::default()
        })
        .with(Character { speed: 10.0 })
        .with(RigidBody2D {
            kinematic: true,
            ..Default::default()
        });
}

fn main() {
    //let half_window_width = WINDOW_WIDTH / 2.0;
    //let half_window_height = WINDOW_HEIGHT / 2.0;

    App::build()
        .add_resource(WindowDescriptor {
            title: "Bevy 2D".to_owned(),
            width: WINDOW_WIDTH as u32,
            height: WINDOW_HEIGHT as u32,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(GameState::default())
        // TODO: this is using some magic knowledge about the size of the viewport
        // (and it's incorrect... so... yeah)
        .add_resource(WorldBounds2D {
            min: Vec2::new(-17.0, -10.0),
            max: Vec2::new(17.0, 10.0),
        })
        .add_resource(WorldConfig::default())
        .add_startup_system(setup.system())
        // add internal camera system update
        .add_system_to_stage(
            bevy::app::stage::POST_UPDATE,
            bevy::render::camera::camera_system::<OrthoProjection>.system(),
        )
        .add_system(character_input_2d_keyboard_system.system())
        .add_system(process_rigidbodies_2d.system())
        .add_system(debug_system.system())
        .run();
}

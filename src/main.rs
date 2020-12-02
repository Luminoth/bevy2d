mod components;
mod debug;
mod game;
mod resources;
mod systems;

use bevy::prelude::*;

use components::character::*;
use components::rigidbody::*;
use game::*;
use resources::world::*;
use systems::character::*;
use systems::physics::*;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

// https://github.com/jamadazi/bevy-cookbook/blob/master/bevy-cookbook.md#custom-camera-projection

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        // cameras
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        // characters
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
            sprite: Sprite::new(Vec2::new(50.0, 50.0)),
            ..Default::default()
        })
        .with(Character { speed: 500.0 })
        .with(RigidBody2D::default());
}

fn main() {
    let half_window_width = WINDOW_WIDTH as f32 / 2.0;
    let half_window_height = WINDOW_HEIGHT as f32 / 2.0;

    App::build()
        .add_resource(WindowDescriptor {
            title: "Bevy 2D".to_owned(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(GameState::default())
        .add_resource(WorldBounds2D {
            min: Vec2::new(-half_window_width, -half_window_height),
            max: Vec2::new(half_window_width, half_window_height),
        })
        .add_resource(WorldConfig::default())
        .add_startup_system(setup.system())
        .add_system(character_input_2d_keyboard_system.system())
        .add_system(process_rigidbodies_2d.system())
        .add_system(debug::debug_system.system())
        .run();
}

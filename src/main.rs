mod components;
mod debug;
mod game;
mod resources;
mod systems;

use bevy::prelude::*;

use components::character::*;
use game::*;
use resources::world::*;
use systems::character::*;

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
        .with(Character { speed: 500.0 });
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Bevy 2D".to_owned(),
            width: 1280,
            height: 720,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(GameState::default())
        .add_resource(WorldBounds2D {
            min: Vec2::new(-500.0, -250.0),
            max: Vec2::new(500.0, 250.0),
        })
        .add_startup_system(setup.system())
        .add_startup_system(debug::debug_world_bounds.system())
        .add_system(character_input_2d_keyboard_system.system())
        .run();
}

//! Herpa derp but this time in TWO dimensions!

mod components;
mod events;
mod resources;
mod states;
mod systems;

use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::physics::RapierPhysicsPlugin;
use bevy_rapier2d::rapier::geometry::InteractionGroups;
use once_cell::sync::Lazy;

use core_lib::components::camera::*;
use core_lib::events::debug::*;
use core_lib::resources::debug::*;
use core_lib::systems::input::*;

use events::character::*;
use states::*;
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

fn setup(asset_server: Res<AssetServer>) {
    #[cfg(debug_assertions)]
    asset_server.watch_for_changes().unwrap();
}

fn setup_debug(mut commands: Commands) {
    commands.insert_resource(DebugState::default());
}

#[bevy_main]
fn main() {
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
        // plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin)
        //.add_plugin(bevy_rapier2d::render::RapierRenderPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        //.add_plugin(LogDiagnosticsPlugin::default())
        // events
        .add_event::<ToggleDebugEvent>()
        .add_event::<JumpEvent>()
        // game states
        .add_state(GameState::Menu)
        .add_system_set(
            SystemSet::on_enter(GameState::Menu)
                .with_system(states::menu::setup.system())
                .with_system(states::menu::setup_ui.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Menu).with_system(states::menu::on_update.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Menu)
                .with_system(states::menu::teardown_ui.system())
                .with_system(states::menu::teardown.system())
                .with_system(core_lib::states::teardown.system()),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(states::game::setup.system())
                .with_system(states::game::setup_world.system())
                .with_system(states::game::setup_ui.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                // input
                .with_system(
                    platformer_2d_keyboard_input
                        .system()
                        .label("character_input"),
                )
                .with_system(character_movement.system().after("character_input"))
                .with_system(jump_input.system().label("character_jump_input"))
                .with_system(character_jump.system().after("character_jump_input"))
                // physics
                .with_system(character_grounded_system.system())
                .with_system(character_gravity_multiplier.system())
                // debug
                .with_system(debug_system.system())
                .with_system(world_bounds_toggle_debug_system.system())
                .with_system(fps_text_system.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Game)
                .with_system(states::game::teardown_ui.system())
                .with_system(states::game::teardown_world.system())
                .with_system(states::game::teardown.system())
                .with_system(core_lib::states::teardown.system()),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::GameOver).with_system(states::gameover::setup.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::GameOver)
                .with_system(states::gameover::teardown.system())
                .with_system(core_lib::states::teardown.system()),
        )
        // setup
        .add_startup_system(setup.system())
        .add_startup_system(setup_debug.system())
        // add internal camera system update
        .add_system_to_stage(
            bevy::app::CoreStage::PostUpdate,
            bevy::render::camera::camera_system::<OrthoProjection>.system(),
        )
        .run();
}

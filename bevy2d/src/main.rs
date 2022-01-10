//! Herpa derp but this time in TWO dimensions!

mod components;
mod events;
mod resources;
mod states;
mod systems;

use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSettings};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorParams, WorldInspectorPlugin};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use once_cell::sync::Lazy;

use core_lib::components::camera::*;
use core_lib::components::character::*;
use core_lib::events::debug::*;
use core_lib::resources::debug::*;
use core_lib::systems::input::*;

use components::game::*;
use events::character::*;
use events::PauseEvent;
use states::*;
use systems::character::*;
use systems::debug::*;
use systems::{pause, pause_input};

// physics layers
const WORLD_LAYER: u32 = 0b01;
const CHARACTER_LAYER: u32 = 0b10;

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
    App::new()
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
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(bevy_rapier2d::render::RapierRenderPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        //.add_plugin(LogDiagnosticsPlugin::default())
        .insert_resource(EguiSettings { scale_factor: 0.8 })
        .add_plugin(EguiPlugin)
        // inspector
        .insert_resource(WorldInspectorParams {
            enabled: false,
            despawnable_entities: true,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<OrthoProjection>()
        .register_inspectable::<TimerText>()
        .register_inspectable::<Character>()
        .register_inspectable::<PlayerCharacter>()
        .register_inspectable::<NonPlayerCharacter>()
        // events
        .add_event::<ToggleDebugEvent>()
        .add_event::<PauseEvent>()
        .add_event::<JumpEvent>()
        // game states
        .add_state(GameState::Menu)
        .add_system_set(
            SystemSet::on_enter(GameState::Menu)
                .with_system(states::menu::setup)
                .with_system(states::menu::setup_ui),
        )
        .add_system_set(SystemSet::on_update(GameState::Menu).with_system(states::menu::on_update))
        .add_system_set(
            SystemSet::on_exit(GameState::Menu)
                .with_system(states::menu::teardown_ui)
                .with_system(states::menu::teardown)
                .with_system(core_lib::states::teardown),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(states::game::setup)
                .with_system(states::game::setup_world)
                .with_system(states::game::setup_ui),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(states::game::on_update)
                .with_system(states::game::update_ui)
                // input
                .with_system(pause_input.label("pause_input"))
                .with_system(pause.label("pause").after("pause_input"))
                .with_system(platformer_2d_keyboard_input.label("character_input"))
                .with_system(character_movement.after("character_input"))
                .with_system(jump_input.label("character_jump_input"))
                .with_system(character_jump.after("character_jump_input"))
                // physics
                .with_system(character_grounded_system)
                .with_system(character_gravity_multiplier)
                .with_system(character_pause.after("pause")),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Game)
                .with_system(states::game::teardown_ui)
                .with_system(states::game::teardown_world)
                .with_system(states::game::teardown)
                .with_system(core_lib::states::teardown),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::GameOver)
                .with_system(states::gameover::setup)
                .with_system(states::gameover::setup_ui),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver).with_system(states::gameover::on_update),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::GameOver)
                .with_system(states::gameover::teardown_ui)
                .with_system(states::gameover::teardown)
                // TODO: this is gross but it's the best "complete" cleanup from the game over sub-state
                .with_system(core_lib::states::teardown),
        )
        // setup
        .add_startup_system(setup)
        .add_startup_system(setup_debug)
        // add internal camera system update
        .add_system_to_stage(
            bevy::app::CoreStage::PostUpdate,
            bevy::render::camera::camera_system::<OrthoProjection>,
        )
        // debug
        .add_system(debug_system)
        .add_system(debug_ui)
        .run();
}

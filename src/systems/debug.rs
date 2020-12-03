//! Debugging systems

use bevy::prelude::*;

use crate::resources::game::*;
use crate::resources::world::*;

fn toggle_debug(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    world_bounds: Res<WorldBounds2D>,
    mut game_state: ResMut<GameState>,
) {
    println!("Toggling debug ...");

    if let Some(mut debug) = game_state.debug.take() {
        debug.cleanup(&mut commands);
    } else {
        game_state.debug = Some(DebugState::new(
            &mut commands,
            &mut meshes,
            &mut materials,
            &world_bounds,
        ));
    }
}

pub fn debug_system(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    world_bounds: Res<WorldBounds2D>,
    game_state: ResMut<GameState>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::D) {
        toggle_debug(commands, meshes, materials, world_bounds, game_state);
    }
}

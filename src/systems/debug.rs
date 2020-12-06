//! Debugging systems

use bevy::diagnostic::*;
use bevy::prelude::*;

use crate::components::debug::*;
use crate::resources::game::*;
use crate::resources::world::*;

fn toggle_debug(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    world_bounds: Res<WorldBounds2D>,
    mut game_state: ResMut<GameState>,
) {
    println!("Toggling debug ...");

    // TODO: what if instead of this
    // we injected and removed debug resources?
    // is that even possible?
    if let Some(mut debug) = game_state.debug.take() {
        debug.cleanup(&mut commands);
    } else {
        game_state.debug = Some(DebugState::new(
            &mut commands,
            &asset_server,
            &mut meshes,
            &mut materials,
            &world_bounds,
        ));
    }
}

pub fn debug_system(
    commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    world_bounds: Res<WorldBounds2D>,
    game_state: ResMut<GameState>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::D) {
        toggle_debug(
            commands,
            asset_server,
            meshes,
            materials,
            world_bounds,
            game_state,
        );
    }
}

pub fn fps_text_system(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    // TODO: should be Wuery<&mut Text, With<FPSText>>
    mut query: Query<(&mut Text, &FPSText)>,
) {
    for (mut text, _) in query.iter_mut() {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_avg) = fps_diagnostic.average() {
                fps = fps_avg;
            }
        }

        let mut frame_time = time.delta_seconds_f64;
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_avg) = frame_time_diagnostic.average() {
                frame_time = frame_time_avg;
            }
        }

        text.value = format!("{:.1} fps, {:.3} ms/frame", fps, frame_time * 1000.0);
    }
}

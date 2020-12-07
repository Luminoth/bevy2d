//! Debugging systems

use bevy::diagnostic::*;
use bevy::prelude::*;

use crate::components::debug::*;
use crate::events::debug::*;
use crate::resources::debug::*;

/// Toggles debug on input
///
/// Sends the ToggleDebugEvent
pub fn debug_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut debug_state: ResMut<DebugState>,
    keyboard_input: Res<Input<KeyCode>>,
    mut debug_events: ResMut<Events<ToggleDebugEvent>>,
) {
    if keyboard_input.just_pressed(KeyCode::D) {
        println!("Toggling debug ...");

        debug_state.enabled = !debug_state.enabled;

        if debug_state.enabled {
            commands
                .spawn(TextComponents {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    text: Text {
                        value: "debug".to_owned(),
                        font: asset_server.load("fonts/Roboto-Regular.ttf"),
                        style: TextStyle {
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    },
                    ..Default::default()
                })
                .with(FPSText);

            debug_state.fps_text_entity = commands.current_entity();
        } else {
            if let Some(fps_text) = debug_state.fps_text_entity.take() {
                commands.despawn(fps_text);
            }
        }

        debug_events.send(ToggleDebugEvent);
    }
}

/// Handles FPS text
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

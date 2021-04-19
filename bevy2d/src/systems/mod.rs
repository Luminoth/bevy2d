//! ECS systems

use bevy::prelude::*;

pub mod character;
pub mod debug;
pub mod world;

use crate::events::PauseEvent;
use crate::states::game::Game;

/// Checks for pause input
pub fn pause_input(keyboard_input: Res<Input<KeyCode>>, mut pause_events: EventWriter<PauseEvent>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        pause_events.send(PauseEvent);
    }
}

/// Handles pause events
pub fn pause(mut game: ResMut<Game>, mut event_reader: EventReader<PauseEvent>) {
    for _ in event_reader.iter() {
        game.paused = !game.paused;
    }
}

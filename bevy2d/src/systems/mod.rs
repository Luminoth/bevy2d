//! ECS systems

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod character;
pub mod debug;

use core_lib::components::character::*;

use crate::events::PauseEvent;
use crate::states::GameState;

/// System labels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum Systems {
    Pause,
    UnPause,
    PauseInput,
    UnPauseInput,

    CharacterInput,
    CharacterJumpInput,
}

/// Checks for pause input
pub fn pause_input(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut pause_events: EventWriter<PauseEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        info!("sending pause event");

        keyboard_input.clear_just_pressed(KeyCode::Escape);

        pause_events.send(PauseEvent);
    }
}

/// Handles pause events
pub fn pause(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    event_reader: EventReader<PauseEvent>,
    query: Query<Entity, With<PlayerCharacter>>,
) {
    if !event_reader.is_empty() {
        info!("pause");

        event_reader.clear();

        for entity in query.iter() {
            commands
                .entity(entity)
                .remove::<RigidBody>()
                .insert(RigidBody::Fixed);
        }

        state.push(GameState::Paused).unwrap();
    }
}

/// Handles unpause events
pub fn unpause(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    event_reader: EventReader<PauseEvent>,
    query: Query<Entity, With<PlayerCharacter>>,
) {
    if !event_reader.is_empty() {
        info!("unpause");

        event_reader.clear();

        for entity in query.iter() {
            commands
                .entity(entity)
                .remove::<RigidBody>()
                .insert(RigidBody::Dynamic);
        }

        state.pop().unwrap();
    }
}

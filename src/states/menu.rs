//! Menu state systems

use bevy::prelude::*;

use super::GameState;

/// Menu update
pub fn on_update(mut state: ResMut<State<GameState>>) {
    state.set(GameState::Game).unwrap();
}

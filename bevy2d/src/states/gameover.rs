//! Game over sub-states

use bevy::prelude::*;

use crate::states::*;

pub struct GameOverTimer(Timer);

/// Game over setup
pub fn setup(mut commands: Commands) {
    commands.insert_resource(GameOverTimer(Timer::from_seconds(10.0, false)));
}

/// Game over tear down
pub fn teardown(mut commands: Commands) {
    commands.remove_resource::<GameOverTimer>();
}

/// Game over state update
pub fn on_update(
    time: Res<Time>,
    mut timer: ResMut<GameOverTimer>,
    mut state: ResMut<State<GameState>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        state.set(GameState::Menu).unwrap();
    }
}

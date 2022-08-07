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

/// Setup the game UI
pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TODO: this should be centered
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                right: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::from_section(
            "Game Over",
            TextStyle {
                font: asset_server.load("fonts/Roboto-Regular.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        ),
        ..Default::default()
    });
}

/// Tear down the game UI
pub fn teardown_ui(mut _commands: Commands) {}

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

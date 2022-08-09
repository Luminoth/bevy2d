//! Game paused sub-state systems

use bevy::prelude::*;

/// Paused setup
pub fn setup(mut _commands: Commands) {}

/// Paused tear down
pub fn teardown(mut _commands: Commands) {}

/// Setup the paused UI
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
            "Paused",
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

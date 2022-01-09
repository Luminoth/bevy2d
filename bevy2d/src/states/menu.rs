//! Menu state systems

use bevy::prelude::*;

use core_lib::components::camera::*;

use crate::CAMERA_SIZE;

use super::GameState;

/// Menu setup
pub fn setup(mut commands: Commands) {
    info!("camera size: {}", CAMERA_SIZE);

    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands
        //.spawn_bundle(OrthographicCameraBundle::new_2d());
        .spawn_bundle(CameraOrtho2dBundle::new(CAMERA_SIZE));
    //.spawn_bundle(CameraOrtho2dBundle::new_2d());
}

/// Menu teardown
pub fn teardown(mut commands: Commands) {
    commands.remove_resource::<ClearColor>();
}

/// Setup the menu UI
pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        margin: Rect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Play",
                            TextStyle {
                                font: asset_server.load("fonts/Roboto-Regular.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
        });

    commands.spawn_bundle(UiCameraBundle::default());
}

/// Tear down the menu UI
pub fn teardown_ui(mut _commands: Commands) {}

/// Menu update
#[allow(clippy::type_complexity)]
pub fn on_update(
    mut query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<GameState>>,
) {
    for (interaction, mut color) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::rgb(0.35, 0.75, 0.35).into();
                state.set(GameState::Game).unwrap();
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

//! Menu state systems

use bevy::prelude::*;

use core_lib::components::camera::*;
use core_lib::resources::button::*;

use crate::CAMERA_SIZE;

use super::GameState;

// TODO: need cleanup functions to despawn all the entities

/// Menu setup
pub fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    info!("camera size: {}", CAMERA_SIZE);

    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    //commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(CameraOrtho2dBundle::new(CAMERA_SIZE));
    //commands.spawn_bundle(CameraOrtho2dBundle::new_2d());

    commands.insert_resource(ButtonMaterials {
        normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
        hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
        pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
    });

    info!("setup done");
}

/// Menu teardown
pub fn teardown(mut commands: Commands) {
    // TODO: camera bundle

    commands.remove_resource::<ClearColor>();
}

/// Setup the menu UI
pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    info!("setup ui");

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Button",
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

    commands.spawn_bundle(UiCameraBundle::default());
}

/// Tear down the menu UI
pub fn teardown_ui(mut commands: Commands) {
    // TODO: camera bundle

    // TODO: button bundle

    commands.remove_resource::<ButtonMaterials>();
}

/// Menu update
pub fn on_update(mut state: ResMut<State<GameState>>) {
    //state.set(GameState::Game).unwrap();
}

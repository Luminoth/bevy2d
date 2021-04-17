//! Menu state systems

use bevy::prelude::*;

use core_lib::components::camera::*;
use core_lib::resources::button::*;

use crate::CAMERA_SIZE;

use super::GameState;

#[derive(Default)]
pub struct MenuEntities {
    camera: Option<Entity>,
}

/// Menu setup
pub fn setup(mut commands: Commands) {
    info!("camera size: {}", CAMERA_SIZE);

    let mut entities = MenuEntities::default();

    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    entities.camera = Some(
        commands
            //spawn_bundle(OrthographicCameraBundle::new_2d());
            .spawn_bundle(CameraOrtho2dBundle::new(CAMERA_SIZE))
            //spawn_bundle(CameraOrtho2dBundle::new_2d());
            .id(),
    );

    commands.insert_resource(entities);
}

/// Menu teardown
pub fn teardown(mut commands: Commands, mut entities: ResMut<MenuEntities>) {
    if let Some(camera) = entities.camera.take() {
        commands.entity(camera).despawn_recursive();
    }

    commands.remove_resource::<MenuEntities>();

    commands.remove_resource::<ClearColor>();
}

#[derive(Default)]
pub struct MenuUIEntities {
    camera: Option<Entity>,
    play_button: Option<Entity>,
}

/// Setup the menu UI
pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let button_materials = ButtonMaterials {
        normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
        hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
        pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
    };

    let mut entities = MenuUIEntities::default();

    entities.play_button = Some(
        commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: button_materials.normal.clone(),
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
            })
            .id(),
    );

    commands.insert_resource(button_materials);

    entities.camera = Some(commands.spawn_bundle(UiCameraBundle::default()).id());

    commands.insert_resource(entities);
}

/// Tear down the menu UI
pub fn teardown_ui(mut commands: Commands, mut entities: ResMut<MenuUIEntities>) {
    if let Some(camera) = entities.camera.take() {
        commands.entity(camera).despawn_recursive();
    }

    if let Some(play_button) = entities.play_button.take() {
        commands.entity(play_button).despawn_recursive();
    }

    commands.remove_resource::<MenuUIEntities>();

    commands.remove_resource::<ButtonMaterials>();
}

/// Menu update
pub fn on_update(
    button_materials: Res<ButtonMaterials>,
    mut query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<State<GameState>>,
) {
    for (interaction, mut material) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                state.set(GameState::Game).unwrap();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

//! Game state resources

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::debug::*;

use super::world::*;

/// Holds the main game state
#[derive(Default)]
pub struct GameState {
    pub debug: Option<DebugState>,
}

/// Holds whatever debug state we need to keep around
pub struct DebugState {
    world_bounds_entity: Option<Entity>,

    fps_text_entity: Option<Entity>,
}

impl DebugState {
    /// Creates a new debug state
    /// Adds all of the resources and entities required for debugging
    pub fn new(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        world_bounds: &Res<WorldBounds2D>,
    ) -> Self {
        commands.spawn(primitive(
            materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
            meshes,
            ShapeType::Rectangle {
                width: world_bounds.width(),
                height: world_bounds.height(),
            },
            TessellationMode::Stroke(&StrokeOptions::default().with_line_width(0.25)),
            world_bounds.min.extend(0.0),
        ));

        let world_bounds_entity = commands.current_entity();

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

        let fps_text_entity = commands.current_entity();

        Self {
            world_bounds_entity,
            fps_text_entity,
        }
    }

    /// Cleans up all of the resources and entities used for debugging
    pub fn cleanup(&mut self, commands: &mut Commands) {
        if let Some(fps_text) = self.fps_text_entity.take() {
            commands.despawn(fps_text);
        }

        if let Some(world_bounds) = self.world_bounds_entity.take() {
            commands.despawn(world_bounds);
        }
    }
}

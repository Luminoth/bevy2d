use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::resources::world::*;

pub fn debug_world_bounds(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    world_bounds: Res<WorldBounds2D>,
) {
    commands.spawn(primitive(
        materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
        &mut meshes,
        ShapeType::Rectangle {
            width: world_bounds.width(),
            height: world_bounds.height(),
        },
        TessellationMode::Stroke(&StrokeOptions::default().with_line_width(3.0)),
        world_bounds.min.extend(0.0),
    ));
}

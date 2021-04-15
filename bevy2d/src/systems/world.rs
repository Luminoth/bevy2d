use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

use core_lib::events::debug::*;
use core_lib::resources::debug::*;

use crate::resources::world::*;

/// Toggls world bounds debugging on event
pub fn world_bounds_toggle_debug_system(
    mut commands: Commands,
    mut event_reader: EventReader<ToggleDebugEvent>,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
    debug_state: Res<DebugState>,
    query: Query<(Entity, &WorldBounds2D)>,
) {
    // TODO: world bounds is a resource not a component
    // so this has to spawn / despawn a new entity

    for _ in event_reader.iter() {
        debug!("toggle world bounds debugging: {}", debug_state.enabled);

        for (entity, world_bounds) in query.iter() {
            debug!("toggle world bounds debugging: {:?}", entity);

            if debug_state.enabled {
                let shape = shapes::Rectangle {
                    width: world_bounds.width(),
                    height: world_bounds.height(),
                    ..Default::default()
                };

                commands.entity(entity).insert(GeometryBuilder::build_as(
                    &shape,
                    ShapeColors::outlined(Color::RED, Color::BLACK),
                    DrawMode::Outlined {
                        fill_options: FillOptions::default(),
                        outline_options: StrokeOptions::default().with_line_width(10.0),
                    },
                    Transform::default(),
                ));
            } else {
                commands.entity(entity).remove::<ShapeBundle>();
            }
        }
    }
}

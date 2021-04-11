use bevy::prelude::*;
//use bevy_prototype_lyon::prelude::*;

use crate::events::debug::*;
use crate::resources::debug::*;
use crate::resources::world::*;

/// Toggls world bounds debugging on event
pub fn world_bounds_toggle_debug_system(
    _commands: Commands,
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

        for (_entity, _world_bounds) in query.iter() {
            /*if debug_state.enabled {
                commands.entity(entity).insert(primitive(
                    materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
                    &mut meshes,
                    ShapeType::Rectangle {
                        width: world_bounds.width(),
                        height: world_bounds.height(),
                    },
                    TessellationMode::Stroke(&StrokeOptions::default().with_line_width(0.25)),
                    world_bounds.min.extend(0.0),
                ));
            } else {
                commands.entity(entity).remove::<SpriteComponents>();
            }*/
        }
    }
}

use bevy::prelude::*;
//use bevy_prototype_lyon::prelude::*;

use crate::events::debug::*;
use crate::resources::debug::*;
use crate::resources::world::*;

/// Toggls world bounds debugging on event
pub fn world_bounds_toggle_debug_system(
    commands: &mut Commands,
    mut event_reader: Local<EventReader<ToggleDebugEvent>>,
    events: Res<Events<ToggleDebugEvent>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    debug_state: Res<DebugState>,
    query: Query<(Entity, &WorldBounds2D)>,
) {
    // TODO: world bounds is a resource not a component
    // so this has to spawn / despawn a new entity

    for _ in event_reader.iter(&events) {
        debug!("toggle world bounds debugging: {}", debug_state.enabled);

        for (entity, world_bounds) in query.iter() {
            /*if debug_state.enabled {
                commands.insert_one(
                    entity,
                    primitive(
                        materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
                        &mut meshes,
                        ShapeType::Rectangle {
                            width: world_bounds.width(),
                            height: world_bounds.height(),
                        },
                        TessellationMode::Stroke(&StrokeOptions::default().with_line_width(0.25)),
                        world_bounds.min.extend(0.0),
                    ),
                );
            } else {
                commands.remove_one::<SpriteComponents>(entity);
            }*/
        }
    }
}

//! Collider systems

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::collider::*;
use crate::events::debug::*;
use crate::resources::debug::*;

/// Toggls collider debugging on event
pub fn collider_toggle_debug_system(
    mut commands: Commands,
    mut event_reader: Local<EventReader<ToggleDebugEvent>>,
    events: Res<Events<ToggleDebugEvent>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    debug_state: Res<DebugState>,
    query: Query<(Entity, &Collider)>,
) {
    for _ in event_reader.iter(&events) {
        println!("Toggle collider debugging: {}", debug_state.enabled);

        for (entity, collider) in query.iter() {
            if debug_state.enabled {
                // TODO: I think this is rendering *behind* the owner
                // so really we probably want to spawn / despawn a child entity

                commands.insert_one(
                    entity,
                    primitive(
                        materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
                        &mut meshes,
                        ShapeType::Rectangle {
                            width: collider.width(),
                            height: collider.height(),
                        },
                        TessellationMode::Stroke(&StrokeOptions::default().with_line_width(0.25)),
                        // TODO: collider translation
                        Vec3::default(),
                    ),
                );
            } else {
                commands.remove_one::<SpriteComponents>(entity);
            }
        }
    }
}

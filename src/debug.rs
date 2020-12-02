use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::game::GameState;
use crate::resources::world::*;

pub struct DebugState {
    world_bounds_entity: Option<Entity>,
}

impl DebugState {
    fn new(
        commands: &mut Commands,
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
            TessellationMode::Stroke(&StrokeOptions::default().with_line_width(3.0)),
            world_bounds.min.extend(0.0),
        ));

        let world_bounds_entity = commands.current_entity();

        Self {
            world_bounds_entity,
        }
    }

    fn cleanup(&mut self, commands: &mut Commands) {
        if let Some(world_bounds) = self.world_bounds_entity.take() {
            commands.despawn(world_bounds);
        }
    }
}

fn toggle_debug(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    world_bounds: Res<WorldBounds2D>,
    mut game_state: ResMut<GameState>,
) {
    println!("Toggling debug ...");

    if let Some(mut debug) = game_state.debug.take() {
        debug.cleanup(&mut commands);
    } else {
        game_state.debug = Some(DebugState::new(
            &mut commands,
            &mut meshes,
            &mut materials,
            &world_bounds,
        ));
    }
}

pub fn debug_system(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    world_bounds: Res<WorldBounds2D>,
    game_state: ResMut<GameState>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::D) {
        toggle_debug(commands, meshes, materials, world_bounds, game_state);
    }
}

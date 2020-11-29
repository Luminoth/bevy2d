mod core;

use bevy::prelude::*;

use crate::core::EnginePlugin;

fn main() {
    App::build()
        .add_plugin(EnginePlugin::new(WindowDescriptor {
            title: "Bevy 2D".to_owned(),
            width: 1280,
            height: 720,
            vsync: true,
            resizable: false,
            ..Default::default()
        }))
        .add_plugins(DefaultPlugins)
        .run();
}

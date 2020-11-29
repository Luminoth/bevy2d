use bevy::prelude::*;

pub struct EnginePlugin {
    window: WindowDescriptor,
}

impl EnginePlugin {
    pub fn new(window: WindowDescriptor) -> Self {
        Self { window }
    }
}

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(self.window.clone())
            .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    }
}

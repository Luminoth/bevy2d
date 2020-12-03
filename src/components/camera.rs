//! Camera components
//!
//! bevy doesn't provide a non-pixel-sized orthographic camera
//! so this one is based on the [cookbook example](https://github.com/jamadazi/bevy-cookbook/blob/master/bevy-cookbook.md#custom-camera-projection)
//! updated to use an "orthographic size" like Unity's orthographic camera

use bevy::prelude::*;
use bevy::render::camera::{Camera, CameraProjection, DepthCalculation, VisibleEntities};
use derivative::Derivative;

#[derive(Derivative, Debug)]
#[derivative(Default)]
pub struct OrthoProjection {
    #[derivative(Default(value = "1.0"))]
    size: f32,

    #[derivative(Default(value = "1000.0"))]
    far: f32,

    #[derivative(Default(value = "1.0"))]
    aspect: f32,
}

impl CameraProjection for OrthoProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(
            -self.aspect * self.size,
            self.aspect * self.size,
            -self.size,
            self.size,
            0.0,
            self.far,
        )
    }

    fn update(&mut self, width: usize, height: usize) {
        self.aspect = width as f32 / height as f32;
    }

    fn depth_calculation(&self) -> DepthCalculation {
        DepthCalculation::ZDifference
    }
}

#[derive(Bundle)]
pub struct Ortho2dComponents {
    pub camera: Camera,
    pub orthographic_projection: OrthoProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Ortho2dComponents {
    pub fn new(size: f32) -> Self {
        // we want 0 to be "closest" and +far to be "farthest" in 2d, so we offset
        // the camera's translation by far and use a right handed coordinate system
        let far = 1000.0;
        Self {
            camera: Camera {
                // have to use one of the internal magic constants
                // since bevy relies on them internally for rendering
                name: Some(bevy::render::render_graph::base::camera::CAMERA2D.to_owned()),
                ..Default::default()
            },
            orthographic_projection: OrthoProjection {
                size,
                far,
                ..Default::default()
            },
            visible_entities: Default::default(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, far - 0.1)),
            global_transform: Default::default(),
        }
    }
}

impl Default for Ortho2dComponents {
    fn default() -> Self {
        Self::new(1.0)
    }
}

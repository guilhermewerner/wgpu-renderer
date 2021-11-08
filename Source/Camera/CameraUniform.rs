use super::Camera;
use bytemuck::{Pod, Zeroable};
use cgmath::SquareMatrix;

// This is so we can store this in a buffer
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn New() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn UpdateViewProjection(&mut self, camera: &Camera) {
        self.view_proj = camera.BuildViewProjectionMatrix().into();
    }
}

/// Matrix to scale and translate from OpenGL coordinate system to WGPU.
#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[path = "Camera.rs"]
mod _Camera;
pub use self::_Camera::*;

#[path = "CameraController.rs"]
mod _CameraController;
pub use self::_CameraController::*;

#[path = "CameraUniform.rs"]
mod _CameraUniform;
pub use self::_CameraUniform::*;

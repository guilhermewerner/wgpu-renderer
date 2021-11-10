use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RgbaColor {
    /// Red component. [0.0, 1.0]
    pub red: f32,

    /// Green component. [0.0, 1.0]
    pub green: f32,

    /// Blue component. [0.0, 1.0]
    pub blue: f32,

    /// Alpha component. [0.0, 1.0]
    pub alpha: f32,
}

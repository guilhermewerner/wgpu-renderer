use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HslaColor {
    /// Hue component. [0.0, 360.0]
    pub hue: f32,

    /// Saturation component. [0.0, 1.0]
    pub saturation: f32,

    /// Lightness component. [0.0, 1.0]
    pub lightness: f32,

    /// Alpha component. [0.0, 1.0]
    pub alpha: f32,
}

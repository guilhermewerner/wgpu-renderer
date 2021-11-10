use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum PolygonMode {
    /// Polygons are filled
    Fill = 0,

    /// Polygons are drawn as line segments
    Line = 1,

    /// Polygons are drawn as points
    Point = 2,
}

impl Default for PolygonMode {
    fn default() -> Self {
        Self::Fill
    }
}

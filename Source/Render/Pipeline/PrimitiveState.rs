use super::{Face, FrontFace, PolygonMode, PrimitiveTopology};
use crate::Render::IndexFormat;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct PrimitiveState {
    /// The primitive topology used to interpret vertices.
    pub topology: PrimitiveTopology,

    /// When drawing strip topologies with indices, this is the required format for the index buffer.
    /// This has no effect on non-indexed or non-strip draws.
    pub strip_index_format: Option<IndexFormat>,

    /// The face to consider the front for the purpose of culling and stencil operations.
    pub front_face: FrontFace,

    /// The face culling mode.
    pub cull_mode: Option<Face>,

    /// Controls the way each polygon is rasterized. Can be either `Fill` (default), `Line` or `Point`
    ///
    /// Setting this to `Line` requires `Features::POLYGON_MODE_LINE` to be enabled.
    ///
    /// Setting this to `Point` requires `Features::POLYGON_MODE_POINT` to be enabled.
    pub polygon_mode: PolygonMode,

    /// If set to true, the polygon depth is clamped to 0-1 range instead of being clipped.
    ///
    /// Enabling this requires `Features::DEPTH_CLAMPING` to be enabled.
    pub clamp_depth: bool,

    /// If set to true, the primitives are rendered with conservative overestimation. I.e. any rastered pixel touched by it is filled.
    /// Only valid for PolygonMode::Fill!
    ///
    /// Enabling this requires `Features::CONSERVATIVE_RASTERIZATION` to be enabled.
    pub conservative: bool,
}

use super::{CompareFunction, DepthBiasState, StencilState, TextureFormat};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DepthStencilState {
    /// Format of the depth/stencil buffer, must be special depth format. Must match the the format
    /// of the depth/stencil attachment in [`CommandEncoder::begin_render_pass`].
    pub format: TextureFormat,

    /// If disabled, depth will not be written to.
    pub depth_write_enabled: bool,

    /// Comparison function used to compare depth values in the depth test.
    pub depth_compare: CompareFunction,

    /// Stencil state.
    pub stencil: StencilState,

    /// Depth bias state.
    pub bias: DepthBiasState,
}

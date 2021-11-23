use super::StencilFaceState;
use serde::{Deserialize, Serialize};

/// State of the stencil operation (fixed-pipeline stage).
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct StencilState {
    /// Front face mode.
    pub front: StencilFaceState,

    /// Back face mode.
    pub back: StencilFaceState,

    /// Stencil values are AND'd with this mask when reading and writing from the stencil buffer. Only low 8 bits are used.
    pub read_mask: u32,

    /// Stencil values are AND'd with this mask when writing to the stencil buffer. Only low 8 bits are used.
    pub write_mask: u32,
}

impl StencilState {
    /// Returns true if the stencil test is enabled.
    pub fn IsEnabled(&self) -> bool {
        (self.front != StencilFaceState::IGNORE || self.back != StencilFaceState::IGNORE)
            && (self.read_mask != 0 || self.write_mask != 0)
    }

    /// Returns true if the state doesn't mutate the target values.
    pub fn IsReadOnly(&self) -> bool {
        self.write_mask == 0
    }

    /// Returns true if the stencil state uses the reference value for testing.
    pub fn NeedsReferenceValue(&self) -> bool {
        self.front.NeedsReferenceValue() || self.back.NeedsReferenceValue()
    }
}

use serde::{Deserialize, Serialize};

/// Operation to perform on the stencil value.
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum StencilOperation {
    /// Keep stencil value unchanged.
    Keep = 0,

    /// Set stencil value to zero.
    Zero = 1,

    /// Replace stencil value with value provided in most recent call to [`RenderPass::set_stencil_reference`].
    Replace = 2,

    /// Bitwise inverts stencil value.
    Invert = 3,

    /// Increments stencil value by one, clamping on overflow.
    IncrementClamp = 4,

    /// Decrements stencil value by one, clamping on underflow.
    DecrementClamp = 5,

    /// Increments stencil value by one, wrapping on overflow.
    IncrementWrap = 6,

    /// Decrements stencil value by one, wrapping on underflow.
    DecrementWrap = 7,
}

impl Default for StencilOperation {
    fn default() -> Self {
        Self::Keep
    }
}

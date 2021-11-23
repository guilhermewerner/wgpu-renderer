use serde::{Deserialize, Serialize};

/// Comparison function used for depth and stencil operations.
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CompareFunction {
    /// Function never passes
    Never = 1,

    /// Function passes if new value less than existing value
    Less = 2,

    /// Function passes if new value is equal to existing value
    Equal = 3,

    /// Function passes if new value is less than or equal to existing value
    LessEqual = 4,

    /// Function passes if new value is greater than existing value
    Greater = 5,

    /// Function passes if new value is not equal to existing value
    NotEqual = 6,

    /// Function passes if new value is greater than or equal to existing value
    GreaterEqual = 7,

    /// Function always passes
    Always = 8,
}

impl CompareFunction {
    /// Returns true if the comparison depends on the reference value.
    pub fn NeedsReferenceValue(self) -> bool {
        match self {
            Self::Never | Self::Always => false,
            _ => true,
        }
    }
}

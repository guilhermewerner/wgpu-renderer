use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum IndexFormat {
    /// Indices are 16 bit unsigned integers.
    UInt16 = 0,

    /// Indices are 32 bit unsigned integers.
    UInt32 = 1,
}

impl Default for IndexFormat {
    fn default() -> Self {
        Self::UInt32
    }
}

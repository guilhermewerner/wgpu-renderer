use serde::{Deserialize, Serialize};

/// Face of a vertex.
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Face {
    /// Front face
    Front = 0,

    /// Back face
    Back = 1,
}

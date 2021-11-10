use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum StepMode {
    Vertex = 0,
    Instance = 1,
}

impl Default for StepMode {
    fn default() -> Self {
        StepMode::Vertex
    }
}

impl From<StepMode> for wgpu::VertexStepMode {
    fn from(step_mode: StepMode) -> Self {
        match step_mode {
            StepMode::Vertex => wgpu::VertexStepMode::Vertex,
            StepMode::Instance => wgpu::VertexStepMode::Instance,
        }
    }
}

use super::{StepMode, VertexAttribute};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct VertexBufferLayout {
    pub label: Cow<'static, str>,
    pub stride: usize,
    pub step_mode: StepMode,
    pub attributes: Vec<VertexAttribute>,
}

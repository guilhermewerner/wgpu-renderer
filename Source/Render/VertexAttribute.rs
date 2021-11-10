use super::VertexFormat;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct VertexAttribute {
    pub label: Cow<'static, str>,
    pub format: VertexFormat,
    pub offset: usize,
    pub shader_location: u32,
}

impl From<VertexAttribute> for wgpu::VertexAttribute {
    fn from(attr: VertexAttribute) -> Self {
        wgpu::VertexAttribute {
            offset: attr.offset as wgpu::BufferAddress,
            shader_location: attr.shader_location,
            format: attr.format.into(),
        }
    }
}

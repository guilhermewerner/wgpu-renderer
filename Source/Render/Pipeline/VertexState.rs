use crate::Render::VertexBufferLayout;
use crate::Shader::Shader;
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct VertexState {
    /// The compiled shader module for this stage.
    pub shader: Cow<'static, Shader>,

    /// The name of the entry point in the compiled shader. There must be a function that returns
    /// void with this name in the shader.
    pub entry_point: &'static str,

    /// The format of any vertex buffers used with this pipeline.
    pub buffers: Cow<'static, [VertexBufferLayout]>,
}

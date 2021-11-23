use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct PipelineDescriptor {
    pub label: Cow<'static, str>,
    pub layout: Option<String>,
    pub shader_stages: String,

}

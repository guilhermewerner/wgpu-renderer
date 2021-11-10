use super::{ShaderSource, ShaderStage};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Shader {
    pub label: Cow<'static, str>,
    pub source: ShaderSource,
    pub stage: ShaderStage,
}

impl Shader {
    pub fn New(stage: ShaderStage, source: ShaderSource) -> Self {
        Self {
            label: "".into(),
            source,
            stage,
        }
    }

    /*
    pub fn FromSpirv(spirv: &[u8]) -> Self {
        Self {
            label: "".into(),
            source: ShaderSource::Spirv(spirv),
            stage,
        }
    }
    */

    pub fn FromGlsl(stage: ShaderStage, glsl: &str) -> Self {
        Self {
            label: "".into(),
            source: ShaderSource::Glsl(glsl.to_string()),
            stage,
        }
    }

    pub fn FromWgsl(wgsl: &str) -> Self {
        Self {
            label: "".into(),
            source: ShaderSource::Wgsl(wgsl.to_string()),
            stage: ShaderStage::Multiple,
        }
    }
}

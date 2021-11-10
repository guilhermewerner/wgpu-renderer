use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ShaderSource {
    Glsl(String),
    Spirv(Vec<u8>),
    Wgsl(String),
}

impl ShaderSource {
    pub fn SpirvFromBytes(bytes: &[u8]) -> ShaderSource {
        ShaderSource::Spirv(Vec::from(bytes))
    }

    pub fn WgslToString(&self) -> Option<&String> {
        if let ShaderSource::Wgsl(s) = &self {
            Some(s)
        } else {
            None
        }
    }
}

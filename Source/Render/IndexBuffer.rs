use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct IndexBuffer {
    pub label: Cow<'static, str>,
    pub content: Vec<u8>,
}

impl IndexBuffer {
    pub fn GetLength(&self) -> u32 {
        self.content.len() as u32
    }
}

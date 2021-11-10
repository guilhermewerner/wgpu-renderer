use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct UniformBuffer {
    pub label: Cow<'static, str>,
    pub content: Vec<u8>,
}

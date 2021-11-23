use crate::Shader::Shader;
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct FragmentState {
    /// The compiled shader module for this stage.
    pub shader: Cow<'static, Shader>,

    /// The name of the entry point in the compiled shader. There must be a function that returns
    /// void with this name in the shader.
    pub entry_point: Cow<'static, str>,

    /*
    /// The color state of the render targets.
    pub targets: &'a [ColorTargetState],
    */
}

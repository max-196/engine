use super::shader::err::ShaderError;

use crate::err::macros::*;


pub enum GpuResourceError {
    Shader(ShaderError),
}

impl_error!(GpuResourceError,
    Shader(e) => "While creating shader: {}", e
);
impl_error_conversion!(GpuResourceError, ShaderError => Shader);
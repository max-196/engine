pub mod err;

pub use err::ShaderError;

use crate::client::renderer::state::State;

pub struct Shader<'a> {
    module: wgpu::ShaderModule,
    ty: ShaderType<'a>,
}

impl <'a> Shader<'a> {
    pub fn import_vert(state: &State, entry: &'a str, path: &std::path::Path, label: &'a str) -> Result<Self, ShaderError> {
        let ty = ShaderType::Vertex(entry);
        Ok(Self::new(
            state,
            label,
            ty,
            wgpu::ShaderSource::Wgsl(crate::files::read_file(std::path::Path::new(path))?.0.into()),
        ))
    }

    pub fn import_frag(state: &State, entry: &'a str, path: &std::path::Path, label: &'a str) -> Result<Self, ShaderError> {
        let ty = ShaderType::Fragment(entry);
        Ok(Self::new(
            state,
            label,
            ty,
            wgpu::ShaderSource::Wgsl(crate::files::read_file(std::path::Path::new(path))?.0.into()),
        ))
    }

    pub fn import_combined(state: &State, entry: (&'a str, &'a str), path: &std::path::Path, label: &'a str) -> Result<Self, ShaderError> {
        let ty = ShaderType::VertexFragment(entry.0, entry.1);
        Ok(Self::new(
            state,
            label,
            ty,
            wgpu::ShaderSource::Wgsl(crate::files::read_file(std::path::Path::new(path))?.0.into()),
        ))
    }


    pub fn new(state: &State, label: &'a str, ty: ShaderType<'a>, source: wgpu::ShaderSource) -> Self {

        let module = state.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source,
        });

        Self {module, ty}
    }

    pub fn vs_state(&self, buffers: &'a [wgpu::VertexBufferLayout]) -> wgpu::VertexState {
        let entry_point = match self.ty {
            ShaderType::Vertex(e) => e,
            ShaderType::Fragment(e) => {
                log::error!("Shader type set as fragment, but tried to get a vertex state! This is probably not what you wanted.");
                e
            },
            ShaderType::VertexFragment(e, _) => e,
        };
        wgpu::VertexState {
            module: &self.module,
            entry_point,
            buffers,
        }
    }

    pub fn fs_state(&self, targets: &'a[Option<wgpu::ColorTargetState>]) -> wgpu::FragmentState {
        let entry_point = match self.ty {
            ShaderType::Vertex(e) => {
                log::error!("Shader type set as vertex, but tried to get a fragment state! This is probably not what you wanted.");
                e
            },
            ShaderType::Fragment(e) => e,
            ShaderType::VertexFragment(_, e) => e,
        };
        wgpu::FragmentState {
            module: &self.module,
            entry_point,
            targets,
        }
    }
}

/// Shader type with entry point(s)
pub enum ShaderType<'a> {
    /// Inner value - vertex entry
    Vertex(&'a str),
    /// Inner value - fragment entry
    Fragment(&'a str),
    /// 1 - Vertex entry, 2 - Fragment entry
    VertexFragment(&'a str, &'a str),
}


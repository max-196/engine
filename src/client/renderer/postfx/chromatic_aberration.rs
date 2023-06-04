use crate::client::renderer::{state::State};

use {
    crate::client::renderer::{
        pipeline::Pipeline,
        gpu::{
            uniform::Uniform,
            shader::Shader,
            err::GpuResourceError,
        },
        framebuffer::FRAMEBUFFER_FORMAT,
    },
    super::PostFx,
};

pub struct ChromaticAberration {
    pipeline: Pipeline,
    offsets: Uniform<[f32; 6]>,
}

impl ChromaticAberration {
    pub fn new(state: &State, framebuffer_layout: &wgpu::BindGroupLayout, off: f32) -> Result<Self, GpuResourceError> {
        let vpath = std::path::Path::new("assets/shaders/framebuffer_vertex.wgsl");
        let fpath = std::path::Path::new("assets/shaders/chrab_fragment.wgsl");

        let vshader = Shader::import_vert(state, "vs_main", vpath, "Framebuffer vertex shader")?;
        let fshader = Shader::import_frag(state, "fs_main", fpath, "Chromatic aberration fragment shader")?;

        let offsets = Uniform::new(&state.device, [-off, -off, 0.0, 0.0, off, off], "Chromatic Aberration", wgpu::ShaderStages::FRAGMENT);

        let pipeline = Pipeline::new(
            state,
            &[framebuffer_layout, offsets.bg.layout()],
            vshader.vs_state(&[]),
            Some(fshader.fs_state(&[Some(wgpu::ColorTargetState {
                format: FRAMEBUFFER_FORMAT,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })])),
            false,
        );

        Ok(Self {pipeline, offsets})
    }
}

impl PostFx for ChromaticAberration {
    fn set_effect<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline.pipeline);
        self.offsets.set(1, render_pass);
    }
    fn label(&self) -> &'static str {"Chromatic Aberration"}
}
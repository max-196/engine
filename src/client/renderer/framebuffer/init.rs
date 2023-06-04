use crate::client::renderer::state::State;

use {
    super::FrameBuffer,
    crate::client::renderer::{
        resources::image::Texture,
        gpu::{
            shader::Shader,
            err::GpuResourceError
        },
        pipeline::Pipeline,
    }
};

impl FrameBuffer {
    pub fn new(state: &State, config: &wgpu::SurfaceConfiguration) -> Result<Self, GpuResourceError> {

        let target_tex = Texture::create_frame_texture(state, config);
        let sample_tex = Texture::create_frame_texture(state, config);

        let vpath = std::path::Path::new("assets/shaders/framebuffer_vertex.wgsl");
        let fpath = std::path::Path::new("assets/shaders/fbuffer_fragment.wgsl");

        let vshader = Shader::import_vert(state, "vs_main", vpath, "Framebuffer vertex shader",)?;
        let fshader = Shader::import_frag(state, "fs_main", fpath, "Framebuffer fragment shader")?;

        let pipeline = Pipeline::new(
            state,
            &[target_tex.bg.layout()],
            vshader.vs_state(&[]),
            Some(fshader.fs_state(&[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })])),
            false,
        );


        Ok(Self {
                    target_tex,
                    sample_tex,
                    pipeline,
                })
    }
}
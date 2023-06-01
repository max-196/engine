mod init;

use crate::client::renderer::{
    resources::image::Texture,
    pipeline::Pipeline,
};

pub const FRAMEBUFFER_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;

pub struct FrameBuffer {
    pub target_tex: Texture,
    pub sample_tex: Texture,
    pipeline: Pipeline,
}

impl FrameBuffer {
    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.target_tex.resize(device, width, height, "Framebuffer texture");
        self.sample_tex.resize(device, width, height, "Framebuffer texture");
    }

    pub fn target_view(&self) -> &wgpu::TextureView {
        &self.target_tex.view
    }

    pub fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline.pipeline
    }

    pub fn sample_bg(&self) -> &wgpu::BindGroup {
        &self.sample_tex.bg.group
    }

    pub fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.target_tex, &mut self.sample_tex);
    }
}

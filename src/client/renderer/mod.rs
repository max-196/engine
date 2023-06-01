pub mod init;
pub mod resources;
pub mod gpu;
pub mod err;
pub mod state;

mod render;
mod pipeline;
mod light;
mod framebuffer;
mod postfx;

pub use err::RendererError;

use {
    crate::instance::Instance,
    resources::{image::{Texture, CubeMap}, model::Model},
    state::State,
    crate::client::Time,
};

pub struct Renderer {
    pub state: State,
    pipeline: pipeline::Pipeline,
    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,
    depth_texture: Texture,
    model: Model,
    light: light::Light,
    framebuffer: framebuffer::FrameBuffer,
    postfx: Vec<Box<dyn postfx::PostFx>>,
    cubemap: CubeMap,
    sky_pipeline: pipeline::Pipeline,
}

impl Renderer {
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.state.resize((new_size.width, new_size.height).into());
            self.depth_texture.resize(&self.state.device, new_size.width, new_size.height, "Depth texture");
            self.framebuffer.resize(&self.state.device, new_size.width, new_size.height);
        }
    }

    pub fn update(&mut self, dt: &Time) {
        self.light.update(&self.state.queue, dt);
    }
}
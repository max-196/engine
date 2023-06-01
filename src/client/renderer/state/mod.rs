pub mod render_state;

mod init;

use crate::common::math::vec::Vec2;

pub use render_state::RenderState;

pub struct State {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: Vec2<u32>,

    pub surface: wgpu::Surface,
    pub config: wgpu::SurfaceConfiguration,
}

impl State {
    pub fn resize(&mut self, new_size: Vec2<u32>) {
        self.size = new_size;
        self.config.width = new_size.x;
        self.config.height = new_size.y;
        self.surface.configure(&self.device, &self.config);
    }
}
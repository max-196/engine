pub mod camera;

use camera::Camera;

use crate::client::Time;

use super::InputManager;

pub struct Player {
    pub camera: Camera,
}

impl Player {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, width: u32, height: u32, input: &mut InputManager) -> Self {
        let mut camera = Camera::new(device, width, height);

        camera.uniform.update(queue, &camera.physical, &camera.projection);

        {
            use winit::event::VirtualKeyCode::*;
            input.register_mapping("forward", W);
            input.register_mapping("backward", S);
            input.register_mapping("left", A);
            input.register_mapping("right", D);
            input.register_mapping("up", Space);
            input.register_mapping("down", LShift);
        }

        Self {camera}
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.camera.projection.resize(width, height);
    }

    pub fn update(&mut self, time: &Time, queue: &wgpu::Queue, inp: &InputManager) {
        self.camera.update(inp);
        self.camera.controller.update_camera(&mut self.camera.physical, time);
        self.camera.uniform.update(queue, &self.camera.physical, &self.camera.projection);
    }

    pub fn camera_bg(&self) -> &wgpu::BindGroup {self.camera.uniform.bg()}
}
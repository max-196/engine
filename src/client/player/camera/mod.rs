mod controller;
mod projection;
mod uniform;
mod physical;

use crate::{common::math::angle::Angle, client::InputManager};


use {
    controller::CameraController,
    projection::Projection,
    uniform::CameraUniform,
    physical::PhysicalCamera,
};

pub struct Camera {
    pub physical:   PhysicalCamera,
    pub projection: Projection,
    pub uniform:    CameraUniform,
    pub controller: CameraController,
}

impl Camera {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let physical = PhysicalCamera::new((0., 1., 0.), 0., 0.);
        let projection = Projection::new(width, height, Angle::from_deg(75.), 0.1, 1000.);
        let uniform = CameraUniform::new(device);
        let controller = CameraController::new(4.0, 0.4);

        Self { physical, projection, uniform, controller }
    }

    pub fn update(&mut self, inp: &InputManager) {
        self.controller.update(inp);
        self.projection.update(inp);
    }
}
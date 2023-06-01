use crate::{common::math::{angle::Angle, mat::Mat4}, client::InputManager};


pub struct Projection {
    aspect: f32,
    fovy: Angle<f32>,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new(
        width: u32,
        height: u32,
        fovy: Angle<f32>,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy,
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> Mat4<f32> {
        Mat4::perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }

    pub fn update(&mut self, input: &InputManager) {
        let s = input.mouse.scroll.y;
        self.fovy += crate::math::Angle::from_deg(s * 0.001);
        self.fovy = self.fovy.clamp(Angle::from_deg(30.), Angle::from_deg(120.));
    }
}

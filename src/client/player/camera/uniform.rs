use crate::client::renderer::gpu::uniform::Uniform;
use crate::common::math::mat::Mat4;
use crate::math::Vec4;


pub struct CameraUniform(Uniform<UniformRaw>);

impl CameraUniform {
    pub fn new(device: &wgpu::Device) -> Self {
        let raw = UniformRaw::new();
        let uniform = Uniform::new(device, raw, "Camera", wgpu::ShaderStages::VERTEX_FRAGMENT);

        Self(uniform)
    }

    pub fn update(&mut self, queue: &wgpu::Queue, camera: &super::PhysicalCamera, projection: &super::Projection) {
        self.0.data.update_view_proj(camera, projection);
        self.0.update(queue);
    }

    pub fn bg(&self) -> &wgpu::BindGroup {&self.0.bg.group}
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct UniformRaw {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
    view_proj_no_translation: [[f32; 4]; 4],
}

impl UniformRaw {
    pub fn new() -> Self {
        Self {
            view_position: [0.; 4],
            view_proj: Mat4::identity().into(),
            view_proj_no_translation: Mat4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &super::PhysicalCamera, projection: &super::Projection) {
        self.view_position = camera.position.homogeneous_point().into();
        let mut cubemap = camera.calc_matrix();
        cubemap.w = Vec4::unit_w();
        self.view_proj_no_translation = (projection.calc_matrix() * cubemap).into();
        self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into();
    }
}
use crate::client::renderer::{
    resources::{
        model::{self, Vertex},
    },
    gpu::{
        shader::Shader,
        uniform::Uniform,
        err::GpuResourceError,
    },
    pipeline::Pipeline,
};

use crate::client::Time;

use super::{state::State};

pub struct Light {
    uniform: Uniform<LightData>,
    pipeline: Pipeline,
}

impl Light {
    pub fn new(state: &State, camera_bgl: &wgpu::BindGroupLayout) -> Result<Self, GpuResourceError> {
        let uniform = Uniform::new(
            &state.device,
            LightData::new(),
            "Light",
            wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
        );

        let vpath = std::path::Path::new("assets/shaders/light_vertex.wgsl");
        let fpath = std::path::Path::new("assets/shaders/light_fragment.wgsl");

        let vshader = Shader::import_vert(state, "vs_main", vpath, "Light vertex shader")?;
        let fshader = Shader::import_frag(state, "fs_main", fpath, "Light fragment shader")?;

        let pipeline = Pipeline::new(
            state,
            &[camera_bgl, uniform.bg.layout()],
            vshader.vs_state(&[model::ModelVertex::desc()]),
            Some(fshader.fs_state(&[Some(wgpu::ColorTargetState {
                format: super::framebuffer::FRAMEBUFFER_FORMAT,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })])),
            true
        );

        Ok(Self {
                    uniform,
                    pipeline,
                })
    }

    pub fn update(&mut self, queue: &wgpu::Queue, time: &Time) {
        use crate::math::{Quat, Vec3, Angle};
        let old_position: Vec3<_> = self.uniform.data.position.into();
        let rot = Quat::from_axis_angle(Vec3::unit_y(), Angle::from_deg(time.dt32 * 150.));
        let rot = rot * old_position;
        self.uniform.data.position = rot.into();
        self.uniform.update(queue);
    }

    pub fn layout(&self) -> &wgpu::BindGroupLayout { self.uniform.bg.layout() }
    pub fn bg(&self) -> &wgpu::BindGroup { &self.uniform.bg.group }
    pub fn pipeline(&self) -> &wgpu::RenderPipeline { &self.pipeline.pipeline }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct LightData {
    position: [f32; 3],
    _padding: u32,
    color: [f32; 3],
    _padding2: u32,
}

impl LightData {
    pub fn new() -> Self {
        Self {
            position: [2., 2., 2.],
            _padding: 0,
            color: [1., 1., 1.],
            _padding2: 0,
        }
    }
}
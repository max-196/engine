use super::Renderer;

use crate::{math::{Zero, Vec3, Quat, Angle}, client::{renderer::{resources::image::{CubeMap, texture::TextureEntry}, pipeline::PipelineBuilder}, PathManager}};

use {
    crate::client::Window,
    super::{
        pipeline,
        err::RendererError,
        framebuffer::FrameBuffer,
        resources::{
            model::{self, Vertex},
            image,
        },
        gpu::shader,
    },
    wgpu::util::DeviceExt,
    crate::instance::Instance,
    super::state::State,
};

const NUM_INSTANCES_PER_ROW: u32 = 10;

impl Renderer {
    pub async fn new(window: &Window, path_m: &PathManager) -> Result<Self, RendererError> {
        let state = State::new(window).await?;


        let instances = Self::init_instances();

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = state.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );


        let depth_texture = image::Texture::create_depth_texture(&state.device, &state.config);

        let vpath = std::path::Path::new("assets/shaders/vertex.wgsl");
        let fpath = std::path::Path::new("assets/shaders/fragment.wgsl");

        let vertex_shader = shader::Shader::import_vert(&state, "vs_main", vpath, "Vertex Geometry Shader")?;
        let fragment_shader = shader::Shader::import_frag(&state, "fs_main",fpath, "Fragment Geometry Shader")?;

        use crate::client::renderer::gpu::uniform::Uniform;
        let uniform_layout_v = Uniform::<u8>::create_layout(&state.device, "Template layout", wgpu::ShaderStages::VERTEX);
        let uniform_layout_vf = Uniform::<u8>::create_layout(&state.device, "Template layout", wgpu::ShaderStages::VERTEX_FRAGMENT);
        let _uniform_layout_f = Uniform::<u8>::create_layout(&state.device, "Template layout", wgpu::ShaderStages::FRAGMENT);

        let material_layout = {
            use super::resources::material::*;
            Material::<()>::layout(&state, &[TextureEntry::DIFFUSE_MAP_ENTRY, TextureEntry::NORMAL_MAP_ENTRY])
        };

        let light = super::light::Light::new(&state, &uniform_layout_vf.0)?;

        let model = model::load_model("assets/models/barrel.obj", &state, path_m)?;

        let pipeline = PipelineBuilder::new(
            vertex_shader.vs_state(&[model::ModelVertex::desc(), crate::instance::InstanceRaw::desc()]),
            Some(fragment_shader.fs_state(&[Some(wgpu::ColorTargetState {
                format: super::framebuffer::FRAMEBUFFER_FORMAT,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })])),
        ).enable_depth().with_bg_layouts(&[&material_layout.0, &uniform_layout_vf.0, light.layout(), &uniform_layout_v.0]).construct(&state);

        let framebuffer = FrameBuffer::new(&state, &state.config)?;

        let postfx: Vec<Box<dyn super::postfx::PostFx>> = vec![
            Box::new(super::postfx::chromatic_aberration::ChromaticAberration::new(&state, framebuffer.target_tex.bg.layout(), 0.005)?),
            Box::new(super::postfx::box_blur::BoxBlur::new(&state, framebuffer.target_tex.bg.layout(), 3)?)
        ];


        let cubemap = CubeMap::from_paths(
            &state,
            path_m,
            [
                    "daylight/Right.png",
                    "daylight/Left.png",
                    "daylight/Top.png",
                    "daylight/Bottom.png",
                    "daylight/Front.png",
                    "daylight/Back.png",
                ],
                "Sky Cubemap"
            )?;

        let sky_shader = shader::Shader::import_combined(&state, ("vs_main", "fs_main"), std::path::Path::new("assets/shaders/sky.wgsl"), "Sky cubemap shader")?;

        let sky_pipeline = PipelineBuilder::new(
            sky_shader.vs_state(&[]),
                Some(sky_shader.fs_state(&[Some(wgpu::ColorTargetState {
                    format: super::framebuffer::FRAMEBUFFER_FORMAT,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })]))
            ).enable_depth().with_depth_compare_function(wgpu::CompareFunction::LessEqual)
            .with_bg_layouts(&[&uniform_layout_vf.0, &cubemap.bg.layout.0])
            .construct(&state);

        log::info!("Renderer configured");
        Ok(Self { state, pipeline, instances, instance_buffer, depth_texture, model, light, framebuffer, postfx, cubemap, sky_pipeline })
    }

    fn init_instances() -> Vec<Instance> {
        const SPACE_BETWEEN: f32 = 3.0;
        (0..NUM_INSTANCES_PER_ROW).flat_map(|z| {
            (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

                let position = Vec3::new(x, 0., z);

                let rotation = if position.is_zero() {
                    Quat::from_axis_angle(Vec3::unit_z(), Angle::zero())
                } else {
                    Quat::from_axis_angle(position.normalize(), Angle::from_deg(45.))
                };

                Instance {
                    position, rotation,
                }
            })
        }).collect::<Vec<_>>()
    }
}
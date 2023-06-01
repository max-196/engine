use crate::client::renderer::resources::image;

use super::state::State;

pub struct Pipeline {
    pub pipeline: wgpu::RenderPipeline,
    _layout: wgpu::PipelineLayout,
}

impl Pipeline {
    pub fn new(state: &State, bg_layouts: &[&wgpu::BindGroupLayout], vertex: wgpu::VertexState, fragment: Option<wgpu::FragmentState>, depth_enabled: bool) -> Self {
        let layout = state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: bg_layouts,
            push_constant_ranges: &[],
        });

        let pipeline = state.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex,
            fragment,
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: if depth_enabled { Some(wgpu::DepthStencilState {
                format: image::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual, // used to use less for everything, lessequal for cubemaps. might return to this!!
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }) } else {None},
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self { pipeline, _layout: layout, }
    }
}
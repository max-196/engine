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

pub struct PipelineBuilder<'a> {
    label: Option<&'a str>,
    bg_layouts: &'a[&'a wgpu::BindGroupLayout],
    vertex_state: Option<wgpu::VertexState<'a>>,
    fragment_state: Option<wgpu::FragmentState<'a>>,
    depth: bool,
    depth_compare: wgpu::CompareFunction,
    depth_write: bool,
    depth_format: wgpu::TextureFormat,
    multisample: bool,
    front_face: wgpu::FrontFace,
    cull: Option<wgpu::Face>,
    polygon_mode: wgpu::PolygonMode,
    topology: wgpu::PrimitiveTopology,
}

impl <'a> PipelineBuilder<'a> {
    pub fn new(vertex_state: wgpu::VertexState<'a>, fragment_state: Option<wgpu::FragmentState<'a>>) -> PipelineBuilder<'a> {
        Self {
            vertex_state: Some(vertex_state),
            fragment_state,
            ..Default::default()
        }
    }

    pub fn with_topology(mut self, t: wgpu::PrimitiveTopology) -> Self {
        self.topology = t; self
    }

    pub fn with_bg_layouts(mut self, layouts: &'a[&'a wgpu::BindGroupLayout]) -> Self {
        self.bg_layouts = layouts; self
    }

    pub fn enable_depth(mut self) -> Self { self.depth = true; self }

    pub fn with_depth_write(mut self, depth_write: bool) -> Self { self.depth_write = depth_write; self }

    pub fn with_depth_compare_function(mut self, f: wgpu::CompareFunction) -> Self { self.depth_compare = f; self }

    pub fn with_polygon_mode(mut self, m: wgpu::PolygonMode) -> Self { self.polygon_mode = m; self }

    pub fn construct(self, state: &State) -> Pipeline {
        let layout = state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: self.label,
            bind_group_layouts: self.bg_layouts,
            push_constant_ranges: &[],
        });

        let pipeline = state.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: self.label,
            layout: Some(&layout),
            vertex: self.vertex_state.unwrap(),
            fragment: self.fragment_state,
            primitive: wgpu::PrimitiveState {
                topology: self.topology,
                strip_index_format: None,
                front_face: self.front_face,
                cull_mode: self.cull,
                polygon_mode: self.polygon_mode,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: if self.depth { Some(wgpu::DepthStencilState {
                format: self.depth_format,
                depth_write_enabled: self.depth_write,
                depth_compare: self.depth_compare,
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

        Pipeline { pipeline, _layout: layout, }
    }
}

impl <'a> std::default::Default for PipelineBuilder<'a> {
    fn default() -> Self {
        Self {
            label: None,
            bg_layouts: &[],
            vertex_state: None,
            fragment_state: None,
            depth: false,
            depth_compare: wgpu::CompareFunction::Less,
            depth_write: true,
            depth_format: image::Texture::DEPTH_FORMAT,
            multisample: false,
            front_face: wgpu::FrontFace::Ccw,
            cull: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            topology: wgpu::PrimitiveTopology::TriangleList,
        }
    }
}
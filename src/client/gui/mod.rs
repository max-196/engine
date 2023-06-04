pub mod element;
pub mod size;
pub mod pos;

pub use {
    element::GuiElement,
    size::Size,
    pos::Pos,
};

use element::Quad;

use crate::math::Vec2;

use super::{renderer::{gpu::{buffer::Buffer, shader::Shader, uniform::Uniform}, Pipeline, state::{State, RenderState}, pipeline::PipelineBuilder, RendererError, resources::model::Vertex}, PathManager, Renderer};

pub struct Gui{
    depth_layers: usize,
    elements: Vec<Vec<GuiElement>>,
    buffers: Vec<Buffer>,
    pipeline: Pipeline,
    uniform: Uniform<UniformRaw>,
}

impl Gui {
    pub fn new(state: &State, path_m: &PathManager, depth_layers: usize) -> Result<Self, RendererError> {
        let mut elements = Vec::with_capacity(depth_layers);

        for _ in 0..depth_layers {
            elements.push(Vec::with_capacity(4));
        }

        let mut buffers = Vec::with_capacity(depth_layers);

        for _ in 0..depth_layers {
            let buf = state.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&format!("GUI Vertex Buffer #{}", depth_layers)),
                size: 1024,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            let vertex_buffer = Buffer(buf);
            buffers.push(vertex_buffer)
        }

        let v = Shader::import_vert(state, "vs_main", path_m.shader("gui/2d_vert.wgsl"), "GUI Vertex Shader")?;
        let f = Shader::import_frag(state, "fs_main", path_m.shader("gui/2d_frag.wgsl"), "GUI Fragment Shader")?;

        let uniform = Uniform::new(&state.device, UniformRaw::new(state.size), "GUI Uniforms", wgpu::ShaderStages::VERTEX_FRAGMENT);

        let pipeline = PipelineBuilder::new(
            v.vs_state(&[Quad::desc()]),
            Some(f.fs_state(&[Some(wgpu::ColorTargetState {
                format: state.config.format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })]))
        ).with_topology(wgpu::PrimitiveTopology::TriangleStrip)
        .with_bg_layouts(&[uniform.bg.layout()])
        .construct(state);

        Ok(Self {
                    depth_layers,
                    elements,
                    buffers,
                    pipeline,
                    uniform,
                })
    }

    pub fn resize(&mut self, state: &State) {
        self.uniform.data = UniformRaw::new(state.size);
        self.uniform.update(&state.queue);

        self.update_verts(state);
    }

    /// Adds an element and updates the corresponding depth layer, if the renderer state is supplied.
    pub fn add(&mut self, element: GuiElement, update_state: Option<&State>) {
        let mut ind = element.depth;
        if ind >= self.depth_layers {
            ind = self.depth_layers - 1;
            log::error!("Tried to set a GUI element with higher depth than allowed by the GUI");
        }
        self.elements[ind].push(element);
        if let Some(s) = update_state {
            self.update_layer_verts(s, ind)
        }
    }

    pub fn update_verts(&mut self, state: &State) {
        for i in 0..self.depth_layers {
            if !self.elements[i].is_empty() {
                self.update_layer_verts(state, i);
            }
        }
    }

    fn update_layer_verts(&mut self, state: &State, depth: usize) {
        let verts: Vec<Quad> = self.elements[depth].iter().map(|e| e.get_raw(state.size)).collect();
        state.queue.write_buffer(&self.buffers[depth].0, 0, bytemuck::cast_slice(&verts));
    }

    pub fn render(&self, render_state: &mut RenderState) {
        let mut render_pass = render_state.render_pass(
            Some("GUI Render Pass"),
            None,
            None,
            None,
        );
        render_pass.set_bind_group(0, &self.uniform.bg.group, &[]);
        for (buffer, elements) in self.buffers.iter().zip(self.elements.iter()) {
            if !elements.is_empty() {
                render_pass.set_pipeline(&self.pipeline.pipeline);
                render_pass.set_vertex_buffer(0, buffer.slice(..));
                render_pass.draw(0..4, 0..(self.elements.len() as u32));
            }
        }
    }
}

#[repr(C)]
#[derive(bytemuck::Pod, Clone, Copy, bytemuck::Zeroable)]
struct UniformRaw {
    pub width: u32,
    pub height: u32,
}

impl UniformRaw {
    pub fn new(size: Vec2<u32>) -> Self {
        Self {width: size.x, height: size.y}
    }
}
use crate::client::renderer::gpu::{
    buffer::Buffer,
    bind_group::{BindGroup, Layout},
};

pub struct Uniform<T> {
    buf: Buffer,
    pub bg: BindGroup,
    pub data: T,
}

impl <T: bytemuck::Pod> Uniform<T> {
    pub fn new(
        device: &wgpu::Device,
        data: T,
        label: &str,
        visibility: wgpu::ShaderStages,
    ) -> Self {

        let buf = Buffer::new_uniform(
            device,
            &[data],
            &(label.to_owned() + " Uniform Buffer"),
        );

        let bg = Self::create_bg(device, label, visibility, &[buf.entry(0)]);

        Self { buf, bg, data }
    }

    pub fn update(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.buf.0, 0, bytemuck::cast_slice(&[self.data]));
    }

    pub fn set<'a>(&'a self, index: u32, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(index, &self.bg.group, &[]);
    }

    pub fn create_layout(device: &wgpu::Device, label: &str, visibility: wgpu::ShaderStages) -> Layout {
        Layout::new(
            device,
            &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            &(label.to_owned() + " Bind Group Layout"),
        )
    }

    fn create_bg(device: &wgpu::Device, label: &str, visibility: wgpu::ShaderStages, group_entries: &[wgpu::BindGroupEntry]) -> BindGroup {
        let layout = Self::create_layout(device, label, visibility);
        BindGroup::with_layout(
            device,
            layout,
            group_entries,
            label
        )
    }
}
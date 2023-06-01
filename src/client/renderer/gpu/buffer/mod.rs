use wgpu::{
    util::DeviceExt,
    BufferUsages,
};

pub struct Buffer(pub wgpu::Buffer);

impl Buffer {
    pub fn new<T: bytemuck::Pod>(device: &wgpu::Device, contents: &[T], label: &str, usage: wgpu::BufferUsages) -> Self {
        Self(device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: bytemuck::cast_slice(contents),
                usage,
            }
        ))
    }

    pub fn new_uniform<T: bytemuck::Pod>(device: &wgpu::Device, contents: &[T], label: &str) -> Self {
        Self::new(
            device,
            contents,
            label,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        )
    }

    pub fn new_vertex<T: bytemuck::Pod>(device: &wgpu::Device, contents: &[T], label: &str) -> Self {
        Self::new(
            device,
            contents,
            label,
            BufferUsages::VERTEX,
        )
    }

    pub fn new_index<T: bytemuck::Pod>(device: &wgpu::Device, contents: &[T], label: &str) -> Self {
        Self::new(
            device,
            contents,
            label,
            BufferUsages::INDEX,
        )
    }

    pub fn entry(&self, binding: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding,
            resource: self.0.as_entire_binding(),
        }
    }

    // pub fn new_quad_index_u32(device: &wgpu::Device, max_vert_count: u32, label: &str) -> Self {
    //     let max_ind_count = (max_vert_count as f32 * 1.5) as u32;
    //     let mut indices: Vec<u32> = Vec::with_capacity(max_ind_count as usize);
    //     for step in (0..max_vert_count).step_by(4) {
    //         indices.push(step);
    //         indices.push(step + 1);
    //         indices.push(step + 2);
    //         indices.push(step + 2);
    //         indices.push(step + 3);
    //         indices.push(step);
    //     }
    //     Self::new_index(device, &indices, label)
    // }

    pub fn slice<S>(&self, bounds: S) -> wgpu::BufferSlice
    where S: std::ops::RangeBounds<wgpu::BufferAddress> {
        self.0.slice(bounds)
    }
}
pub struct RawTexture {
    _tex: wgpu::Texture,
    sampler: wgpu::Sampler,
    view: wgpu::TextureView,
}

impl RawTexture {
    pub fn new(tex: wgpu::Texture, sampler: wgpu::Sampler, view: wgpu::TextureView) -> Self {
        Self {_tex: tex, sampler, view}
    }

    pub fn bind_group_entries(&self, binding: u32) -> (wgpu::BindGroupEntry, wgpu::BindGroupEntry) {
        (wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::TextureView(&self.view),
        },
        wgpu::BindGroupEntry {
            binding: binding + 1,
            resource: wgpu::BindingResource::Sampler(&self.sampler),
        })
    }
}
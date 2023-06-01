use std::num::NonZeroU32;

#[derive(Clone, Copy)]
pub struct TextureEntry {
    visibility: wgpu::ShaderStages,
    multisampled: bool,
    view_dimension: wgpu::TextureViewDimension,
    sample_type: wgpu::TextureSampleType,
    count: Option<NonZeroU32>,
    sampler_binding_type: wgpu::SamplerBindingType,
    pub format: wgpu::TextureFormat,
}

#[allow(unused)]
impl TextureEntry {
    pub const NORMAL_MAP_ENTRY: &TextureEntry = &TextureEntry::new().with_format(wgpu::TextureFormat::Rgba8Unorm);
    pub const DIFFUSE_MAP_ENTRY: &TextureEntry = &TextureEntry::new();

    pub const fn new() -> Self {
        Self {
            visibility: wgpu::ShaderStages::FRAGMENT,
            multisampled: false,
            view_dimension: wgpu::TextureViewDimension::D2,
            sample_type: wgpu::TextureSampleType::Float {filterable: true},
            count: None,
            sampler_binding_type: wgpu::SamplerBindingType::Filtering,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
        }
    }

    pub const fn with_format(mut self, format: wgpu::TextureFormat) -> Self {
        self.format = format; self
    }

    pub const fn with_visibility(mut self, visibility: wgpu::ShaderStages) -> Self {
        self.visibility = visibility; self
    }

    pub const fn with_multisampling(mut self) -> Self {
        self.multisampled = true; self
    }

    pub const fn with_dimension(mut self, view_dimension: wgpu::TextureViewDimension) -> Self {
        self.view_dimension = view_dimension; self
    }

    pub const fn with_sample_type(mut self, sample_type: wgpu::TextureSampleType) -> Self {
        self.sample_type = sample_type; self
    }

    pub const fn with_count(mut self, count: NonZeroU32) -> Self {
        self.count = Some(count); self
    }

    pub const fn with_sampler_binding_type(mut self, ty: wgpu::SamplerBindingType) -> Self {
        self.sampler_binding_type = ty; self
    }

    /// Constructs texture and sampler bind group layout entries
    pub const fn construct(&self, binding: u32) -> (wgpu::BindGroupLayoutEntry, wgpu::BindGroupLayoutEntry) {
        (
            wgpu::BindGroupLayoutEntry {
                binding,
                visibility: self.visibility,
                ty: wgpu::BindingType::Texture {
                    multisampled: self.multisampled,
                    view_dimension: self.view_dimension,
                    sample_type: self.sample_type,
                },
                count: self.count,
            },
            wgpu::BindGroupLayoutEntry {
                binding: binding + 1,
                visibility: self.visibility,
                ty: wgpu::BindingType::Sampler(self.sampler_binding_type),
                count: self.count,
            }
        )
    }
}
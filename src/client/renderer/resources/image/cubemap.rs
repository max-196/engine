use crate::client::{renderer::{gpu::bind_group::BindGroup, state::State}, PathManager};
use super::RawImage;

pub struct CubeMap {
    pub bg: BindGroup,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl CubeMap {

    /// Face order: right, left, top, bottom, front, back
    pub fn from_paths<T: AsRef<std::path::Path>>(state: &State, path_m: &PathManager, src: [T; 6], label: &str) -> Result<Self, super::ImageError> {
        let mut error = None;
        let src = src.map(
            |path| super::RawImage::import_png(path_m.cubemap(path.as_ref()))
            .unwrap_or_else(
                |e| {
                    error = Some(e);
                    RawImage::empty()
                }
        ));
        if let Some(e) = error {return Err(e)}

        Ok(Self::from_raw(state, src, label))
    }

    /// Face order: right, left, top, bottom, front, back
    pub fn from_raw(state: &State, src: [RawImage; 6], label: &str) -> Self {


        let size = src[0].texture_size(6);
        let layer_size = src[0].texture_size(1);

        let desc = wgpu::TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some(label),
            view_formats: &[],
        };

        let texture = state.device.create_texture(&desc);

        for (i, img) in src.iter().enumerate() {
            state.queue.write_texture(
                img.create_copy_tex(&texture, 0, i as u32),
                img.data(),
                img.layout(0),
                layer_size,
            )
        }

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some(label),
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });

        let sampler = state.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bg = BindGroup::new(
            &state.device,
            &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::Cube,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
            }],
            &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label,
        );

        Self {
            bg,
            texture,
            view,
            sampler,
        }
    }
}
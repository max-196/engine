use crate::client::renderer::state::State;

use super::Texture;

use {
    crate::{
        client::renderer::{
            self,
            gpu::bind_group::BindGroup,
        }
    },
};

impl Texture {
    // pub fn from_file(device: &wgpu::Device, queue: &wgpu::Queue, path: &str, label: &str) -> Result<Self, TextureError> {
    //     let (image_data, info) = crate::files::read_texture(Path::new(path))?;
    //     let dimensions = (info.width, info.height);

    //     Ok(Self::from_bytes(device, queue, dimensions, &image_data, label))
    // }

    // fn from_bytes(device: &wgpu::Device, queue: &wgpu::Queue, dimensions: (u32, u32), image_data: &[u8], label: &str) -> Self {
    //     let texture = load_texture(device, queue, dimensions, image_data, label, wgpu::TextureFormat::Rgba8UnormSrgb);

    //     let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    //     let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
    //         address_mode_u: wgpu::AddressMode::ClampToEdge,
    //         address_mode_v: wgpu::AddressMode::ClampToEdge,
    //         address_mode_w: wgpu::AddressMode::ClampToEdge,
    //         mag_filter: wgpu::FilterMode::Nearest,
    //         min_filter: wgpu::FilterMode::Nearest,
    //         mipmap_filter: wgpu::FilterMode::Nearest,
    //         ..Default::default()
    //     });

    //     let bg = BindGroup::new(
    //         device,
    //         &[
    //             wgpu::BindGroupLayoutEntry {
    //                 binding: 0,
    //                 visibility: wgpu::ShaderStages::FRAGMENT,
    //                 ty: wgpu::BindingType::Texture {
    //                     multisampled: false,
    //                     view_dimension: wgpu::TextureViewDimension::D2,
    //                     sample_type: wgpu::TextureSampleType::Float { filterable: true },
    //                 },
    //                 count: None,
    //             },
    //             wgpu::BindGroupLayoutEntry {
    //                 binding: 1,
    //                 visibility: wgpu::ShaderStages::FRAGMENT,
    //                 ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
    //                 count: None,
    //             },
    //         ],
    //         &[
    //             wgpu::BindGroupEntry {
    //                 binding: 0,
    //                 resource: wgpu::BindingResource::TextureView(&view),
    //             },
    //             wgpu::BindGroupEntry {
    //                 binding: 1,
    //                 resource: wgpu::BindingResource::Sampler(&sampler),
    //             }
    //         ],
    //         label
    //     );

    //     Self {bg, texture, view, sampler}
    // }

    pub fn create_depth_texture(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = Self::create_sampler(
            device,
            wgpu::AddressMode::ClampToEdge,
            wgpu::FilterMode::Linear,
            wgpu::FilterMode::Nearest,
            Some(wgpu::CompareFunction::LessEqual)
        );

        let bg = BindGroup::new(
            device,
            &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Depth,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                    count: None,
                },
            ],
            &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                }
            ],
            "Depth Texture"
        );

        Self { bg, texture, view, sampler, desc }
    }

    pub fn create_frame_texture(state: &State, config: &wgpu::SurfaceConfiguration) -> Self {
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some("Framebuffer Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: renderer::framebuffer::FRAMEBUFFER_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = state.device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = Self::create_sampler(
            &state.device,
            wgpu::AddressMode::ClampToEdge,
            wgpu::FilterMode::Linear,
            wgpu::FilterMode::Linear,
            None
        );

        let bg = BindGroup::new(
            &state.device,
            &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                }
            ],
            "Framebuffer texture"
        );

        Self { bg, texture, view, sampler, desc }
    }

    fn create_sampler(
        device: &wgpu::Device,
        address: wgpu::AddressMode,
        scale_filter: wgpu::FilterMode,
        mipmap_filter: wgpu::FilterMode,
        compare: Option<wgpu::CompareFunction>,
    ) -> wgpu::Sampler {
        device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: address,
                address_mode_v: address,
                address_mode_w: address,
                mag_filter: scale_filter,
                min_filter: scale_filter,
                mipmap_filter,
                lod_min_clamp: 0.0,
                lod_max_clamp: 100.0,
                compare,
                ..Default::default()
            }
        )
    }
}
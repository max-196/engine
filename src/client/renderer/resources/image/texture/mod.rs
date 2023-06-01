pub mod entry;
pub mod raw;

pub use entry::TextureEntry;
pub use raw::RawTexture;

mod init;

use crate::client::renderer::gpu::bind_group::BindGroup;

use super::RawImage;
pub use super::err::ImageError;

pub struct Texture {
    pub bg: BindGroup,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    desc: wgpu::TextureDescriptor<'static>,
}

impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32, label: &str) {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        self.desc.size = size;
        self.texture = device.create_texture(&self.desc);
        self.view = self.texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.bg.replace_group(
            device,
            &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&self.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&self.sampler),
            }],
            label
        );
    }
}




pub fn load_texture(device: &wgpu::Device, queue: &wgpu::Queue, image: &RawImage, label: &str, format: wgpu::TextureFormat) -> wgpu::Texture {
    let size = image.texture_size(1);

    let texture = device.create_texture(
        &wgpu::TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some(label),
            view_formats: &[],
        }
    );

    queue.write_texture(
        image.create_copy_tex(&texture, 0, 0),
        image.data(),
        image.layout(0),
        size
    );

    texture
}
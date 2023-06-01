use crate::client::{renderer::{gpu::{bind_group::{BindGroup, Layout}, buffer::Buffer}, state::State}, PathManager};

use super::image::{RawImage, ImageError, texture::{TextureEntry, RawTexture}};

pub struct Material<T: bytemuck::Pod> {
    pub name: String,
    pub bg: BindGroup,
    pub tex: Vec<RawTexture>,
    pub uni: T,
    pub buf: Buffer,
}

impl <T: bytemuck::Pod> Material<T> {
    pub fn from_paths<U: AsRef<std::path::Path>>(state: &State, tex: &[(U, &TextureEntry)], uni: T, label: &str, path_m: &PathManager) -> Result<Self, ImageError> {
        let mut error = None;
        let tex: Vec<_> = tex.iter().map(|(path, entry)| {
            let img = RawImage::import_png(path_m.texture(path.as_ref())).unwrap_or_else(|e| {error = Some(e); RawImage::empty()});
            (img, *entry)
        }).collect();
        if let Some(e) = error { return Err(e) }

        Ok(Self::from_raw_textures(state, tex.as_slice(), uni, label))
    }

    pub fn from_raw_textures(state: &State, tex: &[(RawImage, &TextureEntry)], uni: T, label: &str) -> Self {
        let images: Vec<RawTexture> = tex.iter().enumerate().map(
            |(ind, (tex, entry))| {
                let tex = super::image::texture::load_texture(&state.device, &state.queue, tex, &format!("Image {}", ind), entry.format);
                let sampler = state.device.create_sampler(&wgpu::SamplerDescriptor {
                    address_mode_u: wgpu::AddressMode::ClampToEdge,
                    address_mode_v: wgpu::AddressMode::ClampToEdge,
                    address_mode_w: wgpu::AddressMode::ClampToEdge,
                    mag_filter: wgpu::FilterMode::Nearest,
                    min_filter: wgpu::FilterMode::Nearest,
                    mipmap_filter: wgpu::FilterMode::Nearest,
                    ..Default::default()
                });
                let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
                RawTexture::new(tex, sampler, view)
            }
        ).collect();


        let entries: Vec<_> = tex.iter().map(|(_, e)| *e).collect();
        let layout = Self::layout(state, &entries[..]);

        let mut entries: Vec<_> = images.iter().enumerate().map(|(ind, img)| {
                let entries = img.bind_group_entries(ind as u32 * 2);
                [entries.0, entries.1]
            }
        ).flatten().collect();

        let buf = Buffer::new_uniform(&state.device, &[uni], &(label.to_owned() + " Uniform Buffer"));
        entries.push(buf.entry(entries.len() as u32));

        let bg = BindGroup::with_layout(&state.device, layout, &entries, &format!("{} bind group", label));

        Self { name: label.to_owned(), bg, tex: images, uni, buf }
    }

    pub fn layout(state: &State, tex_entries: &[&TextureEntry]) -> Layout {
        let mut entries = Vec::with_capacity(tex_entries.len() * 2);
        for (ind, entry) in tex_entries.iter().enumerate() {
            let (l1, l2) = entry.construct(ind as u32 * 2);
            entries.push(l1);
            entries.push(l2);
        }
        entries.push(
            wgpu::BindGroupLayoutEntry {
                binding: entries.len() as u32,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
        });
        Layout::new(&state.device, &entries, "Material layout")
    }
}
use crate::math::Vec2;

pub struct RawImage {
    bytes: Vec<u8>,
    pub size: Vec2<u32>,
    line_size: u32,
}

impl RawImage {
    pub fn empty() -> Self { Self { bytes: Vec::new(), size: Vec2::default(), line_size: 0 }}

    pub fn mirror_x(&mut self) {
        let sample_size = (self.line_size / self.size.x) as usize;
        for y in 0..(self.bytes.len() / sample_size) {
            let start = y * sample_size;
            let end = (y + 1) * sample_size;
            let slice = &mut self.bytes[start..end];
            slice.reverse();
        }

        for y in 0..(self.size.y as usize) {
            let start = y * self.line_size as usize;
            let end = start + self.line_size as usize;
            let slice = &mut self.bytes[start..end];
            slice.reverse();
        }
    }

    pub fn import_png<T: AsRef<std::path::Path>>(path: T) -> Result<Self, super::ImageError> {
        let (bytes, norm_info) = crate::files::read_texture(path.as_ref())?;
        let size = (norm_info.width, norm_info.height).into();
        let line_size = norm_info.line_size as u32;

        Ok(Self { bytes, size, line_size })
    }

    pub fn data(&self) -> &[u8] {
        &self.bytes
    }

    pub fn texture_size(&self, depth_or_array_layers: u32) -> wgpu::Extent3d {
        wgpu::Extent3d {
            width: self.size.x,
            height: self.size.y,
            depth_or_array_layers,
        }
    }

    pub fn layout(&self, offset: u64) -> wgpu::ImageDataLayout {
        wgpu::ImageDataLayout {
            offset,
            bytes_per_row: Some(self.line_size),
            rows_per_image: Some(self.size.y),
        }
    }

    pub fn create_copy_tex<'a>(&'a self, texture: &'a wgpu::Texture, mip_level: u32, depth: u32,) -> wgpu::ImageCopyTexture {
        wgpu::ImageCopyTexture {
            texture,
            mip_level,
            origin: wgpu::Origin3d {
                x: 0,
                y: 0,
                z: depth,
            },
            aspect: wgpu::TextureAspect::All,
        }
    }
}
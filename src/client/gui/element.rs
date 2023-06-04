use crate::common::math::{Vec2, Vec4};

use super::{Size, Pos};

pub struct GuiElement {
    origin_type: Origin,
    origin_pos: Pos,
    size: Size,
    min_size: Size,
    max_size: Size,
    pub depth: usize,
    col: Vec4<f32>,
    circle: bool,
    retain_aspect: bool,
}

impl GuiElement {
    pub fn new(depth: usize) -> Self
    {
        return Self {
            origin_type: Origin::TopLeft,
            origin_pos: Pos::Rel2D(Vec2::default()),
            size: Size::Rel2D(Vec2::default()),
            depth,
            col: Vec4::new(1., 1., 1., 1.),
            circle: false,
            min_size: Size::Rel2D(Vec2::default()),
            max_size: Size::Rel2D((1.0, 1.0).into()),
            retain_aspect: true,
        }
    }

    pub fn set_origin(mut self, origin: Origin) -> Self {self.origin_type = origin; self}

    pub fn set_min_size(mut self, size: Size) -> Self {self.min_size = size; self}
    pub fn set_max_size(mut self, size: Size) -> Self {self.max_size = size; self}

    pub fn set_size(mut self, size: Size) -> Self {self.size = size; self}

    pub fn set_pos(mut self, pos: Pos) -> Self {self.origin_pos = pos; self}

    pub fn set_color<T: Into<Vec4<f32>>>(mut self, color: T) -> Self {self.col = color.into(); self}

    pub fn set_circle(mut self, c: bool) -> Self {self.circle = c; self}

    pub fn get_raw(&self, screen_size: Vec2<u32>) -> Quad {
        let o = self.origin_type as u32;

        let mut flags = o << 29;

        flags |= (self.circle as u32) << 28;

        let mut size = self.size.to_screen(screen_size);
        let min_size = self.min_size.to_screen(screen_size);
        let max_size = self.max_size.to_screen(screen_size);

        if self.retain_aspect {
            if size.x < min_size.x && size.y < min_size.y { size = min_size }
            else if size.x > max_size.x && size.y > max_size.y { size = max_size }
        } else {
            if size.x < min_size.x { size.x = min_size.x } else if size.x > max_size.x { size.x = max_size.x }
            if size.y < min_size.y { size.y = min_size.y } else if size.y > max_size.y { size.y = max_size.y }
        }

        Quad {
            flags,
            col: self.col.into(),
            origin: self.origin_pos.to_screen(screen_size).into(),
            size: size.into(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Origin {
    TopLeft = 0,
    BottomLeft = 1,
    TopRight = 2,
    BottomRight = 3,
    Center = 4,
}

#[repr(C)]
#[derive(bytemuck::Pod, Clone, Copy, bytemuck::Zeroable, Default)]
pub struct Quad {
    /// BIT LAYOUT:
    /// 3 - Origin type: 0 - Top left, 1 - Bottom left, 2 - Top right, 3 - Bottom right, 4 - Center
    pub flags: u32,
    pub col: [f32; 4],
    pub origin: [f32; 2],
    pub size: [f32; 2],
}

impl crate::client::renderer::resources::model::Vertex for Quad {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBS: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![0 => Uint32, 1 => Float32x4, 2 => Float32x2, 3 => Float32x2];
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &ATTRIBS,
        }
    }
}
use crate::math::Vec2;

pub enum Pos {
    Pixel2D(Vec2<u32>),
    Rel2D(Vec2<f32>),
}

impl Pos {
    pub fn to_screen(&self, screen_size: Vec2<u32>) -> Vec2<f32> {
        match self {
            Pos::Rel2D(s) => *s,
            Pos::Pixel2D(s) => Vec2::new(
                s.x as f32 / screen_size.x as f32,
                s.y as f32 / screen_size.y as f32,
            )
        }
    }
}
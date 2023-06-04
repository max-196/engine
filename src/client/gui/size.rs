use crate::math::Vec2;

pub enum Size {
    Pixel2D(Vec2<u32>),
    Rel2D(Vec2<f32>),
    PixelSquare(u32),
    RelXSquare(f32),
    RelYSquare(f32),
}

impl Size {
    pub fn to_screen(&self, screen_size: Vec2<u32>) -> Vec2<f32> {
        match self {
            Size::Rel2D(s) => *s,
            Size::Pixel2D(s) => Vec2::new(
                s.x as f32 / screen_size.x as f32,
                s.y as f32 / screen_size.y as f32,
            ),
            Size::PixelSquare(p) => Vec2::new(
                *p as f32 / screen_size.x as f32,
                *p as f32 / screen_size.y as f32,
            ),
            Size::RelXSquare(p) => {
                let aspect = screen_size.x as f32 / screen_size.y as f32;
                Vec2::new(*p, *p * aspect)
            },
            Size::RelYSquare(p) => {
                let aspect = screen_size.x as f32 / screen_size.y as f32;
                Vec2::new(*p / aspect, *p)
            }
        }
    }
}
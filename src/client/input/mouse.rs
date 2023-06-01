use crate::common::math::vec::Vec2;

pub struct Mouse {
    pub mv: Vec2<f64>,
    pub scroll: Vec2<f32>,
    pub button: MouseButtons,
}

impl Mouse {
    pub fn new() -> Self {
        Self {mv: Vec2::default(), scroll: Vec2::default(), button: MouseButtons::new()}
    }

    pub fn update(&mut self) {
        self.mv = Vec2::default();
        self.scroll = Vec2::default();
    }
}

pub struct MouseButtons {
    pub left: bool,
    pub middle: bool,
    pub right: bool,
    pub others: [bool; 32],
}

impl MouseButtons {
    pub fn new() -> Self {
        Self {left: false, middle: false, right: false, others: [false; 32]}
    }
}
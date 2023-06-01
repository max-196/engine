use winit::event::VirtualKeyCode;

pub struct InputMapping {
    pub key: VirtualKeyCode,
    pub pressed: bool,
}

impl InputMapping {
    pub fn new(key: VirtualKeyCode) -> Self {
        Self {key, pressed: false}
    }
}
pub mod mapping;

mod mouse;

use mouse::Mouse;
pub use mapping::InputMapping;

use std::collections::HashMap;

use crate::common::math::vec::Vec2;

use winit::event::{DeviceEvent, MouseScrollDelta, WindowEvent, ElementState, MouseButton, VirtualKeyCode};

pub struct InputManager<'a> {
    mappings: HashMap<&'a str, InputMapping>,
    pub mouse: Mouse,
}

impl <'a> InputManager<'a> {
    pub fn new() -> Self {
        const MAPPING_CAP: usize = 16;
        Self {
            mappings: HashMap::with_capacity(MAPPING_CAP),
            mouse: Mouse::new(),
        }
    }

    pub fn register_mapping(&mut self, name: &'a str, key: VirtualKeyCode) {
        if self.mappings.insert(name, InputMapping::new(key)).is_some() {
            log::warn!("Overwritten mapping {name}")
        }
    }

    /// Returns true if key associated with name is pressed. Returns false otherwise, or if mapping does not exist.
    pub fn get_key(&self, name: &'a str) -> bool {
        match self.mappings.get(name) {
            None => {
                log::error!("No key '{name}' found in mappings."); false
            },
            Some(m) => m.pressed,
        }
    }

    /// Returns true if key associated with name is pressed and sets it to false. Returns false otherwise, or if mapping does not exist.
    pub fn get_key_once(&mut self, name: &'a str) -> bool {
        match self.mappings.get_mut(name) {
            None => {
                log::error!("No key '{name}' found in mappings."); false
            },
            Some(m) => {
                if m.pressed {
                    m.pressed = false;
                    true
                } else {false}
            },
        }
    }

    pub fn device_input(&mut self, event: &DeviceEvent) {
        const SCROLL_FACTOR: f32 = 100.;

        match *event {
            DeviceEvent::MouseMotion { delta } => self.mouse.mv = delta.into(),
            DeviceEvent::MouseWheel { delta } => match delta {
                MouseScrollDelta::LineDelta(x, y) => {
                    //println!("Line delta received: {x} {y}");
                    self.mouse.scroll = Vec2::new(x, y) * SCROLL_FACTOR;
                },
                MouseScrollDelta::PixelDelta(d) => {
                    println!("Pixel delta received: {d:?}");
                    self.mouse.scroll = (d.x as f32, d.y as f32).into();
                }
            },
            DeviceEvent::Key(i) => {
                if let Some(k) = i.virtual_keycode {
                    let pressed = matches!(i.state, ElementState::Pressed);
                    for mapping in self.mappings.values_mut() {
                        if k == mapping.key {mapping.pressed = pressed}
                        //println!("Received key {name}: {pressed}");
                    }
                }
            }
            _ => (),
        }
    }

    pub fn window_input(&mut self, event: &WindowEvent) {
        if let WindowEvent::MouseInput { state, button, .. } = * event {

                let pressed = matches!(state, ElementState::Pressed);
                match button {
                    MouseButton::Left => self.mouse.button.left = pressed,
                    MouseButton::Middle => self.mouse.button.middle = pressed,
                    MouseButton::Right => self.mouse.button.right = pressed,
                    MouseButton::Other(val) => {
                        let val = val as usize;
                        if val < self.mouse.button.others.len() {
                            self.mouse.button.others[val] = pressed;
                        } else {
                            log::error!("Mouse button ID over the button array length");
                        }
                    }
                }
        }
    }

    /// Should be run at the end of the frame;
    /// Sets frame-wide input (mouse) to zero
    pub fn update(&mut self) {
        self.mouse.update();
    }
}
use std::collections::HashMap;

use glium::glutin;
use glutin::event::VirtualKeyCode;

#[allow(dead_code)]
pub struct KeyboardTracker {
    keys: HashMap<VirtualKeyCode, bool>,
}

#[allow(dead_code)]
impl KeyboardTracker {
    pub fn new() -> Self {
        Self {
            keys: HashMap::with_capacity(60),
        }
    }

    pub fn is_key_pressed(&self, keycode: &VirtualKeyCode) -> bool {
        match self.keys.get(keycode) {
            Some(is_pressed) => *is_pressed,
            None => false,
        }
    }

    pub fn set_pressed(&mut self, keycode: VirtualKeyCode, is_pressed: bool) {
        self.keys.insert(keycode, is_pressed);
    }

    pub fn release_all(&mut self) {
        for (_, val) in self.keys.iter_mut() {
            *val = false;
        }
    }

    pub fn is_ctrl_pressed(&self) -> bool {
        self.is_key_pressed(&VirtualKeyCode::LControl) || self.is_key_pressed(&VirtualKeyCode::RControl)
    }

    pub fn is_shift_pressed(&self) -> bool {
        self.is_key_pressed(&VirtualKeyCode::LShift) || self.is_key_pressed(&VirtualKeyCode::RShift)
    }

    pub fn is_alt_pressed(&self) -> bool {
        self.is_key_pressed(&VirtualKeyCode::LAlt) || self.is_key_pressed(&VirtualKeyCode::RAlt)
    }
}
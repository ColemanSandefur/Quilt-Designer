use std::collections::HashMap;

pub struct KeysPressed {
    keys_pressed: HashMap<gdk::keys::Key, bool>,
}

impl KeysPressed {
    pub fn new() -> Self {
        Self {
            keys_pressed: HashMap::with_capacity(64),
        }
    }

    pub fn is_pressed(&self, key: &gdk::keys::Key) -> bool {
        match self.keys_pressed.get(key) {
            Some(is_pressed) => *is_pressed,
            None => false,
        }
    }

    pub fn set_pressed(&mut self, key: gdk::keys::Key, is_pressed: bool) {
        self.keys_pressed.insert(key, is_pressed);
    }

    pub fn release_all(&mut self) {
        for (_, val) in self.keys_pressed.iter_mut() {
            *val = false;
        }
    }
}

pub trait KeyListener { 
    //called when a key is either pressed or released, key_changed is the key that changed if it exists
    fn on_key_change(&self, keys_pressed: &KeysPressed, key_changed: Option<(&gdk::EventKey, bool)>);
}
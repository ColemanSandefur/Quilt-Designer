use crate::window::Window;
use crate::brush::Brush;
use crate::util::keys_pressed::{KeysPressed, KeyListener};

use std::sync::{Arc, Mutex};
use gtk::prelude::*;
use std::ops::Deref;

//
// Texture Bar will hold all the different types of brushes that you might have,
//
// Planning on Texture Bar to consist of
//  - a single color button to choose a specific color that will fill the squares
//  - images of fabrics to fill the squares
//

#[allow(dead_code)]
pub struct TextureBar {
    // main window
    scrolled_window: Arc<Mutex<gtk::ScrolledWindow>>,

    // reference to parent window
    window: Arc<Mutex<Window>>,

    // private fields
    color_button: Arc<Mutex<gtk::ColorButton>>,
}

impl TextureBar {
    pub fn new(window: Arc<Mutex<Window>>) -> Arc<Mutex<Self>> {
        let scrolled_window = Arc::new(Mutex::new(gtk::ScrolledWindowBuilder::new().build()));
        let color_button = Arc::new(Mutex::new(gtk::ColorButton::new()));

        let s = Arc::new(Mutex::new(Self {
            scrolled_window: scrolled_window.clone(),
            color_button: color_button.clone(),
            window: window.clone(),
        }));

        let s_clone = s.clone();
        color_button.lock().unwrap().connect_color_set(move |color_selector| {
            let s_clone = s_clone.lock().unwrap();

            let new_color = color_selector.get_rgba();

            s_clone.set_brush(Arc::new(Brush::new_color((new_color.red, new_color.green, new_color.blue))));
        });

        let scrolled_window = scrolled_window.lock().unwrap();

        scrolled_window.add(color_button.lock().unwrap().deref());

        scrolled_window.show_all();
        
        s
    }

    pub fn get_scrolled_window(&self) -> Arc<Mutex<gtk::ScrolledWindow>> {
        self.scrolled_window.clone()
    }

    fn set_brush(&self, brush: Arc<Brush>) {
        let window = self.window.lock().unwrap();
        let window_brush = window.get_brush();
        let mut window_brush = window_brush.lock().unwrap();

        *window_brush = brush.clone();
    }
}

impl KeyListener for TextureBar {
    fn on_key_change(&self, _keys_pressed: &KeysPressed, key_changed: Option<(&gdk::EventKey, bool)>) {
        let color_button = self.color_button.lock().unwrap();

        if let Some((key_event, is_pressed)) = &key_changed {

            if *is_pressed && key_event.get_keyval() == gdk::keys::constants::space {
                color_button.emit_clicked();
            }

        }
    }
}
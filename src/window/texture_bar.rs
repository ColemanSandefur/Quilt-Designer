use crate::window::Window;

use std::sync::{Arc, Mutex};
use gtk::prelude::*;
use std::ops::Deref;

#[allow(dead_code)]
pub struct TextureBar {
    scrolled_window: Arc<Mutex<gtk::ScrolledWindow>>,

    //reference to parent window
    window: Arc<Mutex<Window>>,
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
            let window = s_clone.window.lock().unwrap();
            let brush = window.get_brush();
            let mut brush = brush.lock().unwrap();

            let new_color = color_selector.get_rgba();

            brush.set_color((new_color.red, new_color.green, new_color.blue));
        });

        let scrolled_window = scrolled_window.lock().unwrap();

        scrolled_window.add(color_button.lock().unwrap().deref());

        scrolled_window.show_all();
        
        s
    }

    pub fn get_scrolled_window(&self) -> Arc<Mutex<gtk::ScrolledWindow>> {
        self.scrolled_window.clone()
    }

    pub fn on_key_press(&self, _application_window: &gtk::ApplicationWindow, event: &gdk::EventKey) -> bool {
        let color_button = self.color_button.lock().unwrap();

        match event.get_keyval().to_unicode() {
            Some(val) => {
                if val == ' ' {
                    color_button.emit_clicked();
            
                    return true;
                }
            },
            None => {}
        }

        false
    }
}
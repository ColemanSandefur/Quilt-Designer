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

    // holds all widgets
    flow_box: Arc<Mutex<gtk::FlowBox>>,

    // reference to parent window
    window: Arc<Mutex<Window>>,

    // private fields
    color_button: Arc<Mutex<gtk::ColorButton>>,

    color_buttons: Vec<Arc::<Brush>>,
}

impl TextureBar {
    fn create_button(window: Arc<Mutex<Window>>, brush: Arc<Brush>) -> gtk::Button {
        let button_builder = gtk::ButtonBuilder::new()
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .height_request(60)
            .width_request(60);
        // let button_builder = button_builder.vexpand_set(false);
            
        let button = button_builder.build();

        let brush_clone = brush.clone();
        let window_clone = window.clone();
        button.connect_clicked(move |_button| {
            let window = window_clone.lock().unwrap();
            let window_brush = window.get_brush();
            let mut window_brush = window_brush.lock().unwrap();

            *window_brush = brush_clone.clone();
        });

        button
    }
    pub fn new(window: Arc<Mutex<Window>>) -> Arc<Mutex<Self>> {
        let scrolled_window_builder = gtk::ScrolledWindowBuilder::new();
        // let scrolled_window_builder = scrolled_window_builder.vexpand_set(false);
        let scrolled_window = Arc::new(Mutex::new(scrolled_window_builder.build()));

        let color_button_builder = gtk::ColorButtonBuilder::new()
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .height_request(60)
            .width_request(60);

        let color_button_builder = match window.lock().unwrap().get_brush().lock().unwrap().get_color() {
            Some(color) => {
                color_button_builder.rgba(&gdk::RGBA {red: color.0, green: color.1, blue: color.2, alpha: 1.0})
            },
            None => color_button_builder.rgba(&gdk::RGBA {red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0})
        };
        

        let color_button = Arc::new(Mutex::new(color_button_builder.build()));

        let flow_box_builder = gtk::FlowBoxBuilder::new()
            .valign(gtk::Align::Start)
            .halign(gtk::Align::Fill);

        let flow_box = Arc::new(Mutex::new(flow_box_builder.build()));

        {
            flow_box.lock().unwrap().set_orientation(gtk::Orientation::Horizontal);
            color_button.lock().unwrap().set_size_request(60, 60);
        }

        let s = Arc::new(Mutex::new(Self {
            scrolled_window: scrolled_window.clone(),
            flow_box: flow_box.clone(),
            color_button: color_button.clone(),
            window: window.clone(),
            color_buttons: Vec::with_capacity(20),
        }));

        // Temporarily adding items to the brush vector
        {
            let mut s = s.lock().unwrap();

            let color_buttons = &mut s.color_buttons;
            
            if let Ok(color) = Brush::try_new_texture("./test_image.jpg") {
                let color = Arc::new(color);
                for _ in 0..30 {
                    color_buttons.push(color.clone());
                }
            }
        }

        let s_clone = s.clone();
        color_button.lock().unwrap().connect_color_set(move |color_selector| {
            let s_clone = s_clone.lock().unwrap();

            let new_color = color_selector.get_rgba();

            s_clone.set_brush(Arc::new(Brush::new_color((new_color.red, new_color.green, new_color.blue))));
        });

        let flow_box = flow_box.lock().unwrap();

        flow_box.add(color_button.lock().unwrap().deref());

        {
            let s = s.lock().unwrap();

            for brush in &s.color_buttons {
                flow_box.add(&TextureBar::create_button(window.clone(), brush.clone()));
            }
        }

        let scrolled_window = scrolled_window.lock().unwrap();

        scrolled_window.add(flow_box.deref());

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
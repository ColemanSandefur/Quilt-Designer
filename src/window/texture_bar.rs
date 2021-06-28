use crate::window::Window;
use crate::texture_brush::TextureBrush;
use crate::util::keys_pressed::{KeysPressed, KeyListener};
use crate::util::image::Image;
use crate::quilt::square::Square;

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

    textures: Vec<Arc<TextureBrush>>,
}

impl TextureBar {
    fn create_button(window: Arc<Mutex<Window>>, brush: Arc<TextureBrush>) -> gtk::Button {
        let button_builder = gtk::ButtonBuilder::new()
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .height_request(60)
            .width_request(60);
            
        let button = button_builder.build();

        let brush_clone = brush.clone();
        let window_clone = window.clone();
        button.connect_clicked(move |_button| {
            let window = window_clone.lock().unwrap();
            let window_brush = window.get_brush();
            let mut window_brush = window_brush.lock().unwrap();

            window_brush.set_texture(brush_clone.clone()); 
        });

        let mut util_image = Image::new(60, 60);
        
        util_image.with_surface(|surface| {
            let cr = cairo::Context::new(surface);
            
            cr.scale(60.0 / Square::SQUARE_WIDTH, 60.0 / Square::SQUARE_WIDTH);
            cr.rectangle(0.0, 0.0, Square::SQUARE_WIDTH, Square::SQUARE_WIDTH);
            brush.apply(&cr);
        });
        
        let image = gtk::Image::from_surface(Some(&util_image.to_surface().unwrap()));
        button.set_image(Some(&image));
        button.set_relief(gtk::ReliefStyle::None);

        button
    }

    pub fn new(window: Arc<Mutex<Window>>) -> Arc<Mutex<Self>> {
        let scrolled_window_builder = gtk::ScrolledWindowBuilder::new();
        let scrolled_window = Arc::new(Mutex::new(scrolled_window_builder.build()));

        let rgb_color;
        let texture_brush = window.lock().unwrap().get_brush().lock().unwrap().get_texture();
        if let Some(brush) = texture_brush {
            if let Some(color) = brush.get_color() {
                rgb_color = gdk::RGBA {red: color.0, green: color.1, blue: color.2, alpha: 1.0};
            } else {
                rgb_color = gdk::RGBA {red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0};
            }
        } else {
            rgb_color = gdk::RGBA {red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0};
        }
        
        let color_button = Arc::new(Mutex::new(gtk::ColorButtonBuilder::new()
            .rgba(&rgb_color)
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .height_request(60)
            .width_request(60)
            .build()
        ));

        let flow_box = Arc::new(Mutex::new(gtk::FlowBoxBuilder::new()
            .valign(gtk::Align::Start)
            .halign(gtk::Align::Fill)
            .orientation(gtk::Orientation::Horizontal)
            .selection_mode(gtk::SelectionMode::None)
            .build()
        ));

        let s = Arc::new(Mutex::new(Self {
            scrolled_window: scrolled_window.clone(),
            flow_box: flow_box.clone(),
            color_button: color_button.clone(),
            window: window.clone(),
            textures: Vec::with_capacity(20),
        }));

        // load textures from ./images and add them to the sidebar
        {
            let mut s = s.lock().unwrap();

            let paths = s.load_textures();
            let textures = &mut s.textures;

            for path in paths {
                if let Ok(color) = TextureBrush::try_new_texture(path.path().to_str().unwrap()) {
                    let color = Arc::new(color);
                    textures.push(color.clone());
                }
            }
        }

        let s_clone = s.clone();
        color_button.lock().unwrap().connect_color_set(move |color_selector| {
            let s_clone = s_clone.lock().unwrap();

            let new_color = color_selector.get_rgba();

            s_clone.set_brush(Arc::new(TextureBrush::new_color((new_color.red, new_color.green, new_color.blue))));
        });

        let flow_box = flow_box.lock().unwrap();

        flow_box.add(color_button.lock().unwrap().deref());

        // create buttons for all of the textures
        {
            let s = s.lock().unwrap();

            for brush in &s.textures {
                flow_box.add(&TextureBar::create_button(window.clone(), brush.clone()));
            }
        }

        let scrolled_window = scrolled_window.lock().unwrap();

        scrolled_window.add(flow_box.deref());

        scrolled_window.show_all();
        
        s
    }

    fn is_image(extension: &std::ffi::OsStr) -> bool {
        let path = extension.to_str().unwrap().to_lowercase();

        match path {
            p if p.eq("png") => true,
            p if p.eq("jpg") => true,
            _ => false
        }
    }

    fn load_textures(&mut self) -> std::vec::Vec<std::fs::DirEntry> {
        let path = std::path::Path::new("./images");

        let mut results = Vec::with_capacity(20);

        if path.is_dir() {
            for entry in std::fs::read_dir(path).unwrap() {
                let entry = entry.unwrap();
                let file_name = entry.file_name();
                let extension = std::path::Path::new(&file_name).extension().unwrap();

                if TextureBar::is_image(extension) {
                    results.push(entry);
                }
            }
        }

        results
    }

    pub fn get_scrolled_window(&self) -> Arc<Mutex<gtk::ScrolledWindow>> {
        self.scrolled_window.clone()
    }

    fn set_brush(&self, brush: Arc<TextureBrush>) {
        let window = self.window.lock().unwrap();
        let window_brush = window.get_brush();
        let mut window_brush = window_brush.lock().unwrap();

        window_brush.set_texture(brush.clone()); 
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
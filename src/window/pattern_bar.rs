use crate::window::Window;
use crate::util::keys_pressed::{KeysPressed, KeyListener};
use crate::quilt::square::Square;
use crate::quilt::block_pattern::BlockPattern;
use crate::util::image::Image;
use crate::util::parser::SavableBlueprint;

use std::sync::{Arc, Mutex};
use gtk::prelude::*;
use std::ops::Deref;
use std::io::Read;

//
// Texture Bar will hold all the different types of brushes that you might have,
//
// Planning on Texture Bar to consist of
//  - a single color button to choose a specific color that will fill the squares
//  - images of fabrics to fill the squares
//

#[allow(dead_code)]
pub struct PatternBar {
    // main window
    scrolled_window: Arc<Mutex<gtk::ScrolledWindow>>,

    // holds all widgets
    flow_box: Arc<Mutex<gtk::FlowBox>>,

    // reference to parent window
    window: Arc<Mutex<Window>>,

    // private fields
    patterns: Vec<BlockPattern>,
}

impl PatternBar {
    fn create_button(window: Arc<Mutex<Window>>, brush: BlockPattern) -> gtk::Button {
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

            window_brush.set_block_pattern(brush_clone.clone());
        });

        let mut util_image = Image::new(60, 60);
        
        util_image.with_surface(|surface| {
            let cr = cairo::Context::new(surface).unwrap();
            
            cr.scale(60.0 / Square::SQUARE_WIDTH, 60.0 / Square::SQUARE_WIDTH);
            cr.set_source_rgb(0.25, 0.25, 0.25);
            cr.paint().unwrap();
            brush.draw(&cr);
        });
            
        let image = gtk::Image::from_surface(Some(&util_image.to_surface().unwrap()));
        button.set_image(Some(&image));
        button.set_relief(gtk::ReliefStyle::None);

        button
    }

    pub fn new(window: Arc<Mutex<Window>>) -> Arc<Mutex<Self>> {
        let scrolled_window_builder = gtk::ScrolledWindowBuilder::new();
        let scrolled_window = Arc::new(Mutex::new(scrolled_window_builder.build()));

        let flow_box_builder = gtk::FlowBoxBuilder::new()
            .valign(gtk::Align::Start)
            .halign(gtk::Align::Fill);

        let flow_box = Arc::new(Mutex::new(flow_box_builder.build()));

        {
            flow_box.lock().unwrap().set_orientation(gtk::Orientation::Horizontal);
            flow_box.lock().unwrap().set_selection_mode(gtk::SelectionMode::None);
        }

        let s = Arc::new(Mutex::new(PatternBar {
            scrolled_window: scrolled_window.clone(),
            flow_box: flow_box.clone(),
            window: window.clone(),
            patterns: Vec::with_capacity(20),
        }));

        let flow_box = flow_box.lock().unwrap();

        {
            let mut s = s.lock().unwrap();

            s.load_patterns();

            for brush in &s.patterns {
                flow_box.add(&PatternBar::create_button(window.clone(), brush.clone()));
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

    fn is_pattern(extension: &std::ffi::OsStr) -> bool {
        let path = extension.to_str().unwrap().to_lowercase();

        match path {
            p if p.eq("yaml") => true,
            _ => false
        }
    }

    fn load_pattern_yaml(path: &str) -> BlockPattern {
        let mut file = std::fs::File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let yaml = &yaml_rust::YamlLoader::load_from_str(&contents).unwrap()[0];

        *BlockPattern::from_save_blueprint(yaml)
    }

    fn load_patterns(&mut self) {
        let patterns = &mut self.patterns;

        let path = std::path::Path::new("./patterns");

        if path.is_dir() {
            for entry in std::fs::read_dir(path).unwrap() {
                let entry = entry.unwrap();
                let file_name = entry.file_name();
                let extension = std::path::Path::new(&file_name).extension().unwrap();

                if PatternBar::is_pattern(extension) {
                    patterns.push(PatternBar::load_pattern_yaml(&entry.path().to_str().unwrap()));
                }
            }
        }
    }
}

impl KeyListener for PatternBar {
    fn on_key_change(&self, _keys_pressed: &KeysPressed, _key_changed: Option<(&gdk::EventKey, bool)>) {
    }
}
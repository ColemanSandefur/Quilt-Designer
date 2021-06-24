use crate::window::Window;
// use crate::brush::Brush;
use crate::util::keys_pressed::{KeysPressed, KeyListener};
use crate::quilt::square::{BlockPattern, Square};
use crate::quilt::child_shape;
use crate::util::image::Image;

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
pub struct PatternBar {
    // main window
    scrolled_window: Arc<Mutex<gtk::ScrolledWindow>>,

    // holds all widgets
    flow_box: Arc<Mutex<gtk::FlowBox>>,

    // reference to parent window
    window: Arc<Mutex<Window>>,

    // private fields

    patterns: Vec<BlockPattern>,
    pattern_buttons: Vec<Arc::<gtk::Button>>,
}

impl PatternBar {
    fn create_button(window: Arc<Mutex<Window>>, brush: BlockPattern) -> gtk::Button {
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

            // *window_brush = brush_clone.clone();
            window_brush.set_block_pattern(brush_clone.clone());
        });

        let mut util_image = Image::new(60, 60);
        
        util_image.with_surface(|surface| {
            let cr = cairo::Context::new(surface);
            
            cr.scale(60.0 / Square::SQUARE_WIDTH, 60.0 / Square::SQUARE_WIDTH);
            cr.set_source_rgb(0.25, 0.25, 0.25);
            cr.paint();
            brush.draw(&cr);
        });
            
        let image = gtk::Image::from_surface(Some(&util_image.to_surface().unwrap()));
        button.set_image(Some(&image));
        button.set_relief(gtk::ReliefStyle::None);

        button
    }

    pub fn new(window: Arc<Mutex<Window>>) -> Arc<Mutex<Self>> {
        let scrolled_window_builder = gtk::ScrolledWindowBuilder::new();
        // let scrolled_window_builder = scrolled_window_builder.vexpand_set(false);
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
            pattern_buttons: Vec::with_capacity(20),
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

    fn load_patterns(&mut self) {
        let patterns = &mut self.patterns;

        let mid_point = Square::SQUARE_WIDTH / 2.0;
        let width = Square::SQUARE_WIDTH;

        // empty pattern
        patterns.push(BlockPattern::new_pattern(vec![]));

        // 4 squares
        patterns.push(BlockPattern::new_pattern(vec![
            child_shape::prefab::create_rect(0.0, 0.0, 
                mid_point, mid_point),
            child_shape::prefab::create_rect(mid_point, 0.0, 
                mid_point, mid_point),
            child_shape::prefab::create_rect(mid_point, mid_point, 
                mid_point, mid_point),
            child_shape::prefab::create_rect(0.0, mid_point, 
                mid_point, mid_point),
        ]));
        
        // 1 square in the corner
        patterns.push(BlockPattern::new_pattern(vec![
            child_shape::prefab::create_rect(0.0, 0.0, 
                mid_point, mid_point),
        ]));
        
        // half square triangle
        patterns.push(BlockPattern::new_pattern(vec![
            child_shape::prefab::create_triangle((0.0, 0.0), (0.0, width), (width, 0.0))
        ]));


        // 4 triangles
        patterns.push(BlockPattern::new_pattern(vec![
            child_shape::prefab::create_triangle((0.0, 0.0), (mid_point, mid_point), (width, 0.0)),
            child_shape::prefab::create_triangle((width, 0.0), (mid_point, mid_point), (width, width)),
            child_shape::prefab::create_triangle((width, width), (mid_point, mid_point), (0.0, width)),
            child_shape::prefab::create_triangle((0.0, width), (mid_point, mid_point), (0.0, 0.0)),
        ]))
    }
}

impl KeyListener for PatternBar {
    fn on_key_change(&self, _keys_pressed: &KeysPressed, _key_changed: Option<(&gdk::EventKey, bool)>) {
    }
}
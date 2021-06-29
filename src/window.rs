pub mod texture_bar;
pub mod canvas;
pub mod pattern_bar;

use crate::texture_brush::TextureBrush;
use crate::brush::Brush;
use crate::util::keys_pressed::{KeysPressed, KeyListener};
use canvas::Canvas;
use texture_bar::TextureBar;
use pattern_bar::PatternBar;

use gdk::EventMask;
use glib::signal::Inhibit;
use std::sync::{Arc, Mutex};
use gtk::prelude::*;
use std::ops::Deref;
use std::f64::consts::FRAC_PI_2;

//
// The main window of the application
//
// Will handle all the layout stuff and will handle key press events
// it holds references to its children in the main struct
//

#[allow(dead_code)]
pub struct Window {
    window: Arc<Mutex<gtk::ApplicationWindow>>,

    // child widgets
    canvas: Option<Arc<Mutex<Canvas>>>,
    texture_bar: Option<Arc<Mutex<TextureBar>>>,
    pattern_bar: Option<Arc<Mutex<PatternBar>>>,
    
    // local fields
    brush: Arc<Mutex<Brush>>,
    keys_pressed: Arc<Mutex<KeysPressed>>,
}

impl Window {
    

    pub fn new(application: &gtk::Application) -> Arc<Mutex<Self>> {
        // create variables before putting it into a window
        // just in case I want to use them later
        let window = Arc::new(Mutex::new(gtk::ApplicationWindow::new(application)));

        let brush = Arc::new(Mutex::new(Brush::with_texture(Arc::new(TextureBrush::new()))));
        let keys_pressed = Arc::new(Mutex::new(KeysPressed::new()));

        let s = Arc::new(Mutex::new(Self {
            window: Arc::clone(&window),

            canvas: None,
            texture_bar: None,
            pattern_bar: None,
            
            brush: Arc::clone(&brush),
            keys_pressed: Arc::clone(&keys_pressed),
        }));

        // circular dependency from my bad code, we have to create the Window struct before we create the canvas
        // we then assign the canvas to the window
        let canvas = Canvas::new(Arc::clone(&s));
        s.lock().unwrap().canvas = Some(canvas);

        let texture_bar = TextureBar::new(s.clone());
        s.lock().unwrap().texture_bar = Some(texture_bar);

        let pattern_bar = PatternBar::new(s.clone());
        s.lock().unwrap().pattern_bar = Some(pattern_bar);

        //
        // Event handlers
        //

        let window = window.lock().unwrap();
        
        let s_clone = Arc::clone(&s);
        window.add_events(EventMask::KEY_PRESS_MASK);
        window.connect_key_press_event(move |window, event| {
            s_clone.lock().unwrap().on_key_press(window, event)
        });

        let s_clone = Arc::clone(&s);
        window.add_events(EventMask::KEY_RELEASE_MASK);
        window.connect_key_release_event(move |window, event| {
            s_clone.lock().unwrap().on_key_release(window, event)
        });

        let s_clone = Arc::clone(&s);
        window.add_events(EventMask::FOCUS_CHANGE_MASK);
        window.connect_focus_out_event(move |window, event| {
            s_clone.lock().unwrap().on_focus_out(window, event)
        });


        window.set_default_size(600, 500);

        let s_clone = Arc::clone(&s);
        s_clone.lock().unwrap().set_up_layout(window.as_ref());
        drop(s_clone);

        window.show_all();

        s
    }

    fn set_up_layout(&self, window: &gtk::ApplicationWindow) {
        // each paned widget can only have 2 children
        // so we need 2 panes to hold 3 widgets (left bar, drawing area, right bar)

        // will contain left_bar, and drawing area
        let paned = gtk::Paned::new(gtk::Orientation::Horizontal);

        // will hold the first paned widget and right_bar
        let paned2 = gtk::Paned::new(gtk::Orientation::Horizontal);

        let drawing_area = self.canvas.as_ref().unwrap().lock().unwrap().get_drawing_area();
        let left_bar = self.texture_bar.as_ref().unwrap().lock().unwrap().get_scrolled_window();
        let right_bar = self.pattern_bar.as_ref().unwrap().lock().unwrap().get_scrolled_window();

        left_bar.lock().unwrap().set_size_request(100, 500);
        drawing_area.lock().unwrap().set_size_request(400, 500);

        paned.pack1(left_bar.lock().unwrap().deref(), false, false);
        paned.pack2(drawing_area.lock().unwrap().deref(), true, true);

        paned.set_size_request(500, 500);
        right_bar.lock().unwrap().set_size_request(100, 500);

        paned2.pack1(&paned, true, false);
        paned2.pack2(right_bar.lock().unwrap().deref(), false, false);
        window.add(&paned2);
    }

    fn on_key_press(&self, _application_window: &gtk::ApplicationWindow, event: &gdk::EventKey) -> Inhibit {

        {
            let mut keys_pressed_unlocked = self.keys_pressed.lock().unwrap();

            keys_pressed_unlocked.set_pressed(event.keyval(), true);


            let pattern = {self.brush.lock().unwrap().get_block_pattern()};
            if let Some(mut brush) =  pattern {
                if keys_pressed_unlocked.is_pressed(&gdk::keys::constants::r) {
                    brush.rotate(FRAC_PI_2);
                } else if keys_pressed_unlocked.is_pressed(&gdk::keys::constants::R) {
                    brush.rotate(-FRAC_PI_2);
                }

                self.brush.lock().unwrap().set_block_pattern(brush);
            }

            if let Some(canvas) = &self.canvas {
                canvas.lock().unwrap().on_key_change(&keys_pressed_unlocked, Some((event, true)));
            }

            if let Some(texture_bar) = &self.texture_bar {
                texture_bar.lock().unwrap().on_key_change(&keys_pressed_unlocked, Some((event, true)));
            }

            if let Some(pattern_bar) = &self.pattern_bar {
                pattern_bar.lock().unwrap().on_key_change(&keys_pressed_unlocked, Some((event, true)));
            }
        }

        Inhibit(false)
    }

    fn on_key_release(&self, _application_window: &gtk::ApplicationWindow, event: &gdk::EventKey) -> Inhibit {

        {
            let mut keys_pressed_unlocked = self.keys_pressed.lock().unwrap();

            keys_pressed_unlocked.set_pressed(event.keyval(), false);

            if let Some(canvas) = &self.canvas {
                canvas.lock().unwrap().on_key_change(&keys_pressed_unlocked, Some((event, false)));
            }
        }

        Inhibit(false)
    }

    fn on_focus_out(&self, _application_window: &gtk::ApplicationWindow, _event: &gdk::EventFocus) -> Inhibit {
        let mut keys_pressed_unlocked = self.keys_pressed.lock().unwrap();

        keys_pressed_unlocked.release_all();

        if let Some(canvas) = &self.canvas {
            canvas.lock().unwrap().on_key_change(&keys_pressed_unlocked, None);
        }

        Inhibit(false)
    }

    pub fn get_brush(&self) -> Arc<Mutex<Brush>> {
        Arc::clone(&self.brush)
    }

    pub fn get_keys_pressed(&self) -> Arc<Mutex<KeysPressed>> {
        Arc::clone(&self.keys_pressed)
    }
}
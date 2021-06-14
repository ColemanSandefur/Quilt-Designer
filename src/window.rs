pub mod texture_bar;
pub mod canvas;

use crate::brush::Brush;
use canvas::Canvas;
use texture_bar::TextureBar;

use gdk::EventMask;
use glib::signal::Inhibit;
use std::sync::{Arc, Mutex};
use gtk::prelude::*;
use std::ops::Deref;

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
    
    // local fields
    brush: Arc<Mutex<Arc<Brush>>>, // brush is immutable so we need to change the reference
}

impl Window {
    pub fn new(application: &gtk::Application) -> Arc<Mutex<Self>> {
        // create variables before putting it into a window
        // just in case I want to use them later
        let window = Arc::new(Mutex::new(gtk::ApplicationWindow::new(application)));

        let brush = Arc::new(Mutex::new(Arc::new(Brush::new())));

        let s = Arc::new(Mutex::new(Self {
            window: Arc::clone(&window),

            canvas: None,
            texture_bar: None,
            
            brush: Arc::clone(&brush),
        }));

        // circular dependency from my bad code, we have to create the Window struct before we create the canvas
        // we then assign the canvas to the window
        let canvas = Canvas::new(Arc::clone(&s));
        s.lock().unwrap().canvas = Some(canvas);

        let texture_bar = TextureBar::new(s.clone());
        s.lock().unwrap().texture_bar = Some(texture_bar);

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
        let right_bar = Arc::new(Mutex::new(gtk::ScrolledWindowBuilder::new().build()));

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

    fn on_key_press(&self, application_window: &gtk::ApplicationWindow, event: &gdk::EventKey) -> Inhibit {

        if let Some(canvas) = &self.canvas {
            canvas.lock().unwrap().on_key_press(application_window, event);
        }

        if let Some(texture_bar) = &self.texture_bar {
            texture_bar.lock().unwrap().on_key_press(application_window, event);
        }

        Inhibit(false)
    }

    fn on_key_release(&self, application_window: &gtk::ApplicationWindow, event: &gdk::EventKey) -> Inhibit {

        if let Some(canvas) = &self.canvas {
            canvas.lock().unwrap().on_key_release(application_window, event);
        }

        Inhibit(false)
    }

    pub fn get_brush(&self) -> Arc<Mutex<Arc<Brush>>> {
        Arc::clone(&self.brush)
    }
}
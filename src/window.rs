use crate::brush::Brush;
use crate::canvas::Canvas;

use gdk::EventMask;
use glib::signal::Inhibit;
use std::sync::{Arc, Mutex};
use gtk::DrawingArea;
use gtk::prelude::*;
use std::ops::Deref;

#[allow(dead_code)]
pub struct Window {
    window: Arc<Mutex<gtk::ApplicationWindow>>,
    drawing_area: Arc<Mutex<DrawingArea>>,
    canvas: Option<Arc<Mutex<Canvas>>>,
    
    brush: Arc<Mutex<Brush>>,
}

impl Window {
    pub fn new(application: &gtk::Application) -> Arc<Mutex<Self>> {
        //create variables before putting it into a window
        //just in case I want to use them later
        let window = Arc::new(Mutex::new(gtk::ApplicationWindow::new(application)));
        let drawing_area = Arc::new(Mutex::new(Box::new(DrawingArea::new)()));
        // let canvas = Canvas::new(Arc::clone(&window), Arc::clone(&drawing_area));
        let brush = Arc::new(Mutex::new(Brush::new()));

        let s = Arc::new(Mutex::new(Self {
            window: Arc::clone(&window),
            drawing_area: Arc::clone(&drawing_area),
            canvas: None,
            
            brush: Arc::clone(&brush),
        }));

        let canvas = Canvas::new(Arc::clone(&s), Arc::clone(&drawing_area));
        s.lock().unwrap().canvas = Some(canvas);

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

        window.set_default_size(500, 500);
        window.add(drawing_area.lock().unwrap().deref());
        window.show_all();

        s
    }

    fn on_key_press(&self, application_window: &gtk::ApplicationWindow, event: &gdk::EventKey) -> Inhibit {

        if let Some(canvas) = &self.canvas {
            canvas.lock().unwrap().on_key_press(application_window, event);
        }

        Inhibit(false)
    }

    fn on_key_release(&self, application_window: &gtk::ApplicationWindow, event: &gdk::EventKey) -> Inhibit {

        if let Some(canvas) = &self.canvas {
            canvas.lock().unwrap().on_key_release(application_window, event);
        }

        Inhibit(false)
    }

    pub fn get_brush(&self) -> Arc<Mutex<Brush>> {
        Arc::clone(&self.brush)
    }
}
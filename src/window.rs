use crate::quilt::Quilt;

use std::sync::{Arc, Mutex};
use gtk::DrawingArea;
use gtk::prelude::*;
use gdk::{EventButton, EventMask};
use glib::signal::Inhibit;
use std::collections::VecDeque;

#[allow(dead_code)]
pub struct Window {
    window: Arc<gtk::ApplicationWindow>,
    drawing_area: Arc<DrawingArea>,
    quilt: Arc<Mutex<Quilt>>,
    mouse_clicks: Arc<Mutex<VecDeque<(f64, f64)>>>,
}

impl Window {
    pub fn new(application: &gtk::Application) -> Arc<Self> {
        let window = Arc::new(gtk::ApplicationWindow::new(application));
        let drawing_area = Arc::new(Box::new(DrawingArea::new)());
        let quilt = Arc::new(Mutex::new(
            Quilt::new(5, 6)
        ));
        let mouse_clicks = Arc::new(Mutex::new(VecDeque::with_capacity(10)));

        let s = Arc::new(Self {
            window: Arc::clone(&window),
            drawing_area: Arc::clone(&drawing_area),
            quilt: Arc::clone(&quilt),
            mouse_clicks: Arc::clone(&mouse_clicks),
        });

        //s_clone is always moved into a closure
        let s_clone = Arc::clone(&s);
        drawing_area.connect_draw(move |drawing_area, cr| {
            s_clone.draw(drawing_area, cr)
        });

        let s_clone = Arc::clone(&s);
        drawing_area.add_events(EventMask::BUTTON_PRESS_MASK);
        drawing_area.connect_button_press_event(move |drawing_area, event| {
            s_clone.on_click(drawing_area, event)
        });

        window.set_default_size(500, 500);

        window.add(drawing_area.as_ref());
        window.show_all();

        s
    }

    fn draw(&self, _drawing_area: &DrawingArea, cr: &cairo::Context) -> Inhibit {
        cr.translate(200.0, 200.0);
        
        let mut mouse_clicks = self.mouse_clicks.lock().unwrap();

        while !mouse_clicks.is_empty() {
            let (x, y) = mouse_clicks.pop_front().unwrap();
            println!("{:?}", cr.device_to_user(x, y));
        }

        drop(mouse_clicks);

        let quilt = self.quilt.lock().unwrap();
        quilt.draw(cr);

        Inhibit(false)
    }

    fn on_click(&self, drawing_area: &DrawingArea, event: &EventButton) -> Inhibit {
        let mut mouse_clicks = self.mouse_clicks.lock().unwrap();

        mouse_clicks.push_back(event.get_position());
        drop(mouse_clicks);

        drawing_area.queue_draw();

        Inhibit(false)
    }
}
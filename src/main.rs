extern crate gtk;
pub mod quilt;

use std::env;
use quilt::Quilt;

use gtk::prelude::*;
use gtk::DrawingArea;
use gio::prelude::*;
use gdk::EventMask;
use std::sync::{Arc, Mutex};

fn build_ui(application: &gtk::Application, width: i32, height: i32) {

    let data: Arc<Mutex<(f64, f64)>> = Arc::new(Mutex::new((0.0, 0.0)));
    let quilt: Arc<Mutex<Quilt>> = Arc::new(Mutex::new(
        Quilt::new(5, 6)
    ));

    let window = gtk::ApplicationWindow::new(application);
    let drawing_area = Box::new(DrawingArea::new)();

    //gets moved to the button press event handler
    let data_button_press = Arc::clone(&data);

    drawing_area.add_events(EventMask::BUTTON_PRESS_MASK);
    drawing_area.connect("button_press_event", true, move |values| {
        let drawing_area = &values[0].get::<gtk::DrawingArea>().unwrap().unwrap();
        let raw_event = &values[1].get::<gdk::Event>().unwrap().unwrap();

        match raw_event.downcast_ref::<gdk::EventButton>() {
            Some(event) => {
                let mut x = data_button_press.lock().unwrap();
                *x = event.get_position();
                drawing_area.queue_draw();
            }
            None => {},
        }

        let res = glib::value::Value::from_type(glib::types::Type::Bool);
        Some(res)
    }).unwrap();

    //moved to draw function
    let quilt_draw = Arc::clone(&quilt);

    //the drawing function for the drawing area
    drawing_area.connect_draw(move |_drawing_area, cr| {
        cr.translate(200.0, 200.0);

        let loc = data.lock().unwrap();

        println!("{:?}", cr.device_to_user(loc.0, loc.1));

        let quilt = quilt_draw.lock().unwrap();
        quilt.draw(cr);

        Inhibit(false)
    });

    window.set_default_size(width, height);

    window.add(&drawing_area);
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("org.gtkrsnotes.demo"),
        gio::ApplicationFlags::FLAGS_NONE)
        .expect("Application::new failed");

    application.connect_activate(|application| {
        build_ui(application, 500, 500);
    });

    application.run(&env::args().collect::<Vec<_>>());
}
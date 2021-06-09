extern crate gtk;
mod quilt;

use std::env;
use quilt::Quilt;

use gtk::prelude::*;
use gtk::DrawingArea;
use gio::prelude::*;
use gdk::EventMask;
use cairo::{Context};

static mut location: (f64, f64) = (0.0, 0.0);

fn build_ui(application: &gtk::Application) {

    drawable(application, 500 + 60, 500, |drawing_area, cr| {
        
        // let menu_width = 20.0;
        // cr.move_to(0.0, 0.0);
        // cr.line_to(menu_width, 0.0);
        // cr.line_to(menu_width, 500.0);
        // cr.line_to(0.0, 500.0);
        // cr.set_source_rgb(0.0, 0.0, 0.0);
        // cr.fill();
        // cr.translate(menu_width, 0f64);
        // cr.scale(500f64, 500f64);
        // cr.translate(offset_x, 0f64);

        cr.translate(200.0, 200.0);
        
        drawing_area.add_events(EventMask::BUTTON_PRESS_MASK);
        
        drawing_area.connect("button_press_event", true, |values| {

            let drawing_area = &values[0].get::<gtk::DrawingArea>().unwrap().unwrap();

            let raw_event = &values[1].get::<gdk::Event>().unwrap().unwrap();

            match raw_event.downcast_ref::<gdk::EventButton>() {
                Some(event) => {
                    println!("{}", event.get_button());
                    unsafe {location = event.get_position();};
                    drawing_area.queue_draw();
                }
                None => {},
            }

            let res = glib::value::Value::from_type(glib::types::Type::Bool);
            Some(res)
        }).unwrap();

        unsafe{ println!("{:?}", cr.device_to_user(location.0, location.1)); }
        let quilt = Quilt::new(5, 6);
        quilt.draw(cr);

        Inhibit(false)
    });
}

fn main() {
    let application = gtk::Application::new(Some("org.gtkrsnotes.demo"),
    gio::ApplicationFlags::FLAGS_NONE)
.expect("Application::new failed");

    application.connect_activate(build_ui);

    // build_ui(&application);

    application.run(&env::args().collect::<Vec<_>>());
}

pub fn drawable<F>(application: &gtk::Application, width: i32, height: i32, draw_fn: F)
where
    F: Fn(&DrawingArea, &Context) -> Inhibit + 'static,
{
    let window = gtk::ApplicationWindow::new(application);
    let drawing_area = Box::new(DrawingArea::new)();

    drawing_area.connect_draw(draw_fn);

    // drawing_area.add_events();

    window.set_default_size(width, height);

    window.add(&drawing_area);
    window.show_all();
}
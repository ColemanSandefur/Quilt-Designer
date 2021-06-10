extern crate gtk;
pub mod quilt;
pub mod window;
pub mod frame_timing;

use std::env;
use gio::prelude::*;

fn build_ui(application: &gtk::Application) {
    let _window = window::Window::new(application);
}

fn main() {
    let application = gtk::Application::new(Some("org.gtkrsnotes.demo"),
        gio::ApplicationFlags::FLAGS_NONE)
        .expect("Application::new failed");

    application.connect_activate(build_ui);

    application.run(&env::args().collect::<Vec<_>>());
}
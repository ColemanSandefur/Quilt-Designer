extern crate gtk;
pub mod quilt;
pub mod window;
pub mod camera_transform;
pub mod texture_brush;
pub mod brush;
pub mod util;
pub mod path;
pub mod parser;
pub mod menubar_manager;

use gio::prelude::*;
use gtk::prelude::*;
use menubar_manager::MenubarManager;

fn build_ui(application: &gtk::Application) {
    let mut menubar = MenubarManager::new();
    application.set_menubar(Some(menubar.get_menubar()));

    let window = window::Window::new(application);

    menubar.set_window(Some(window.clone()));
    menubar.load_menubar_actions(application);
}

fn main() {
    let application = gtk::Application::new(Some("org.gtkrsnotes.demo"),
        gio::ApplicationFlags::FLAGS_NONE);

    application.connect_activate(build_ui);

    application.run();
}
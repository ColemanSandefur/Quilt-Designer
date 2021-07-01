extern crate gtk;
pub mod quilt;
pub mod window;
pub mod camera_transform;
pub mod util;
pub mod menubar_manager;
pub mod draw;

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
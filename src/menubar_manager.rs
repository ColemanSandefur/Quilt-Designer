use gio::{Menu, MenuItem, SimpleAction};
use gtk::Application;
use std::sync::{Arc, Mutex};
use gio::prelude::*;
use crate::window::Window;

pub struct MenubarManager {
    menubar: Menu,
    window: Option<Arc<Mutex<Window>>>
}

impl MenubarManager {
    pub fn new() -> Self {
        Self {
            menubar: Self::create_menubar(),
            window: None,
        }
    }

    pub fn get_menubar(&self) -> &Menu {
        &self.menubar
    }

    pub fn get_menubar_mut(&mut self) -> &mut Menu {
        &mut self.menubar
    }

    pub fn set_window(&mut self, window: Option<Arc<Mutex<Window>>>) {
        self.window = window;
    }

    fn create_menubar() -> Menu {
        let menu = Menu::new();
    
        menu.append_item(&Self::create_file_selection());
    
        menu
    }
    
    fn create_file_selection() -> MenuItem {
        let sub_menu = Menu::new();

        sub_menu.append(Some("Save"), Some("app.save"));
    
        MenuItem::new_submenu(Some("file"), &sub_menu)
    }

    pub fn load_menubar_actions(&self, application: &Application) {
        // let menubar_clone = self.menubar.clone();
        self.connect_activate("save", None, application, move |_action, _variant| {
            // menubar_clone
            println!("Save clicked");
        });
    }

    // helper method to create actions
    fn connect_activate<F: Fn(&SimpleAction, Option<&glib::Variant>) + 'static>(&self, name: &str, parameter_type: Option<&glib::VariantTy>, application: &Application, f: F,) {
        let save_action = SimpleAction::new(name, parameter_type);
        save_action.connect_activate(f);
        application.add_action(&save_action);
    }
}

use gio::{Menu, MenuItem, SimpleAction};
use gtk::Application;
use std::sync::{Arc, Mutex};
use gio::prelude::*;
use crate::window::Window;
use gtk::prelude::*;

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
        sub_menu.append(Some("Open"), Some("app.open"));
    
        MenuItem::new_submenu(Some("file"), &sub_menu)
    }

    pub fn load_menubar_actions(&self, application: &Application) {

        let window_clone = self.window.as_ref().unwrap().clone();
        self.connect_activate("save", None, application, move |_action, _variant| {
            let app_window = window_clone.lock().unwrap().get_window();

            let file_chooser = gtk::FileChooserDialog::with_buttons(
                Some("Save your quilt"),
                Some(&*app_window.lock().unwrap()),
                gtk::FileChooserAction::Save,
                &[("Cancel", gtk::ResponseType::Cancel), ("Save", gtk::ResponseType::Accept)]
            );

            file_chooser.set_select_multiple(false);
            let filter = gtk::FileFilter::new();
            filter.add_pattern("*.quilt");
            file_chooser.set_filter(&filter);
            file_chooser.set_do_overwrite_confirmation(true);

            // sets the chosen file's extension to .quilt
            // currently silently overwrites if filename + extension exists
            file_chooser.connect_file_activated(|file_chooser| {
                if let Some(mut filename) = file_chooser.filename() {
                    filename.set_extension("quilt");

                    file_chooser.select_filename(&filename);
                }
            });

            let result = file_chooser.run();

            file_chooser.emit_close();

            let mut file_path = match result {
                gtk::ResponseType::Accept => file_chooser.file().unwrap().path().unwrap(),
                _ => return
            };

            file_path.set_extension("quilt");

            let canvas = window_clone.lock().unwrap().get_canvas().unwrap();
            canvas.lock().unwrap().save(&file_path);
        });

        let window_clone = self.window.as_ref().unwrap().clone();
        self.connect_activate("open", None, application, move |_action, _variant| {
            let app_window = window_clone.lock().unwrap().get_window();

            let file_chooser = gtk::FileChooserDialog::with_buttons(
                Some("Choose a file to open"),
                Some(&*app_window.lock().unwrap()),
                gtk::FileChooserAction::Open,
                &[("Cancel", gtk::ResponseType::Cancel), ("Open", gtk::ResponseType::Accept)]
            );

            file_chooser.set_select_multiple(false);
            let filter = gtk::FileFilter::new();
            filter.add_pattern("*.quilt");
            file_chooser.set_filter(&filter);
            let result = file_chooser.run();

            file_chooser.emit_close();

            let file_path = match result {
                gtk::ResponseType::Accept => file_chooser.file().unwrap().path().unwrap(),
                _ => return
            };
            
            let canvas = window_clone.lock().unwrap().get_canvas().unwrap();

            canvas.lock().unwrap().load(&file_path);
        });
    }

    // helper method to create actions
    fn connect_activate<F: Fn(&SimpleAction, Option<&glib::Variant>) + 'static>(&self, name: &str, parameter_type: Option<&glib::VariantTy>, application: &Application, f: F,) {
        let save_action = SimpleAction::new(name, parameter_type);
        save_action.connect_activate(f);
        application.add_action(&save_action);
    }
}

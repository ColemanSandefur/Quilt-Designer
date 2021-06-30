// #[cfg(feature = "dox")]

use crate::camera_transform::CameraTransform;
use crate::util::frame_timing::FrameTiming;
use crate::util::click::Click;
use crate::util::keys_pressed::{KeysPressed, KeyListener};
use crate::util::rectangle::Rectangle;
use crate::quilt::{Quilt, square::Square};
use crate::window::Window;
use crate::util::undo_redo::UndoRedo;
use crate::parser::Savable;

use cairo::Context;
use gdk::{EventMask, ScrollDirection};
use glib::signal::Inhibit;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use gtk::DrawingArea;
use gtk::prelude::*;
use std::io::{Read, Write};

//
// Main drawing window
//
// It will draw anything that I need drawn,
//     as of right now I am planning on just drawing the quilt
//
// It also handles things like camera movement, click events and frame timings
//

#[allow(dead_code)]
pub struct Canvas {
    drawing_area: Arc<Mutex<DrawingArea>>,

    // reference to parent window
    window: Arc<Mutex<Window>>,

    // local fields
    mouse_clicks: Arc<Mutex<VecDeque<gdk::EventButton>>>,
    quilt: Arc<Mutex<Quilt>>,
    camera_transform: Arc<Mutex<CameraTransform>>,
    frame_timing: Arc<Mutex<FrameTiming>>,
    saved_surface: Arc<Mutex<Option<cairo::ImageSurface>>>,
    needs_updated: Arc<Mutex<bool>>,
    undo_redo: Arc<Mutex<UndoRedo<Square>>>,
}

impl Canvas {
    pub fn new(window: Arc<Mutex<Window>>) -> Arc<Mutex<Self>> {
        let drawing_area = Arc::new(Mutex::new(Box::new(DrawingArea::new)()));
        let quilt = Arc::new(Mutex::new(Quilt::new(5,6)));
        let camera_transform = Arc::new(Mutex::new(CameraTransform::new()));
        let frame_timing = Arc::new(Mutex::new(FrameTiming::new()));
        let mouse_clicks = Arc::new(Mutex::new(VecDeque::with_capacity(10)));
        let saved_surface = Arc::new(Mutex::new(None));

        let s = Arc::new(Mutex::new(Self {
            window: Arc::clone(&window),
            drawing_area: Arc::clone(&drawing_area),

            quilt: Arc::clone(&quilt),
            camera_transform: Arc::clone(&camera_transform),
            frame_timing: Arc::clone(&frame_timing),
            mouse_clicks: Arc::clone(&mouse_clicks),
            saved_surface: saved_surface.clone(),
            needs_updated: Arc::new(Mutex::new(true)),
            undo_redo: Arc::new(Mutex::new(UndoRedo::new())),
        }));

        let drawing_area = drawing_area.lock().unwrap();

        let s_clone = Arc::clone(&s);
        drawing_area.connect_draw(move |drawing_area, cr| {
            s_clone.lock().unwrap().draw(drawing_area, cr)
        });

        let s_clone = Arc::clone(&s);
        drawing_area.add_events(EventMask::BUTTON_PRESS_MASK);
        drawing_area.connect_button_press_event(move |drawing_area, event| {
            s_clone.lock().unwrap().on_click(drawing_area, event)
        });

        let s_clone = Arc::clone(&s);
        drawing_area.add_events(EventMask::SCROLL_MASK);
        drawing_area.connect_scroll_event(move |drawing_area, event| {
            s_clone.lock().unwrap().on_scroll(drawing_area, event)
        });

        camera_transform.lock().unwrap().offset = (20.0, 20.0);

        s
    }

    fn pre_draw(&self, drawing_area: &DrawingArea, cr: &Context) {

        cr.save().unwrap();

        let width = drawing_area.allocated_width() as f64;
        let height = drawing_area.allocated_height() as f64;

        cr.move_to(0.0, 0.0);
        cr.line_to(width, 0.0);
        cr.line_to(width, height);
        cr.line_to(0.0, height);
        cr.line_to(0.0, 0.0);
        cr.clip();

        // will handle any necessary camera movements and apply them
        self.camera_transform.lock().unwrap().apply_offset(cr);
        
        // should handle clicks before draw
        self.handle_clicks(cr);
    }

    
    fn draw(&self, drawing_area: &DrawingArea, cr: &Context) -> Inhibit {
        cr.save().unwrap();

        // before any rendering
        self.pre_draw(drawing_area, cr);

        // I changed the rendering to draw to a surface, save the surface, and then render the surface on the cairo context
        // this helps improve the frame timings when you are just panning around.
        // I need to address the main rendering though (when you modify the square or change the zoom), it is still very slow
        let mut saved_surface = self.saved_surface.lock().unwrap();

        if *self.needs_updated.lock().unwrap() == true {
            let (raw_width, raw_height) = self.quilt.lock().unwrap().get_dimensions_pixel();
            let zoom = self.camera_transform.lock().unwrap().get_zoom();
            let (width, height) = (raw_width * zoom, raw_height * zoom);

            match cairo::ImageSurface::create(cairo::Format::ARgb32, width as i32, height as i32) {
                Ok(surface) => {
                    *saved_surface = Some(surface);
                },
                Err(err) => {
                    println!("ERROR: {:?}", err);
                }
            }

            *self.needs_updated.lock().unwrap() = false;
        }

        let (offset_x, offset_y) = self.camera_transform.lock().unwrap().get_offset();

        let bounds = Rectangle {
            x: offset_x, 
            y: offset_y, 
            width: drawing_area.allocated_width() as f64, 
            height: drawing_area.allocated_height() as f64
        };

        if let Some(surface) = &saved_surface.as_ref() {
            let new_context = cairo::Context::new(surface).unwrap();
            self.camera_transform.lock().unwrap().apply_zoom(&new_context);
            self.quilt.lock().unwrap().draw(&new_context, self.camera_transform.clone(), &bounds);
            cr.set_source_surface(surface, 0.0, 0.0).unwrap();
            cr.paint().unwrap();
        } else {
            *self.needs_updated.lock().unwrap() = true;
        }

        self.post_draw(drawing_area, cr);
        cr.restore().unwrap();

        Inhibit(false)
    }

    fn post_draw(&self, drawing_area: &DrawingArea, cr: &Context) {
        let mut frame_timing = self.frame_timing.lock().unwrap();

        cr.restore().unwrap();

        // draw frame timing in top left corner
        cr.save().unwrap();

        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.set_font_size(14.0);
        cr.move_to(4.0, 18.0);
        cr.show_text(&format!("{}ms", frame_timing.delta_frame_time().num_milliseconds())).unwrap();

        cr.restore().unwrap();

        self.camera_transform.lock().unwrap().move_with_keys_pressed(&frame_timing.delta_frame_time());

        // update the last recorded time we rendered a frame
        frame_timing.update_frame_time();
        drawing_area.queue_draw();
    }

    // will add any clicks that drawing area receives and add it to a queue for handle_clicks to use on next draw
    fn on_click(&self, _drawing_area: &DrawingArea, event: &gdk::EventButton) -> Inhibit {
        let mut mouse_clicks = self.mouse_clicks.lock().unwrap();

        mouse_clicks.push_back(event.to_owned());
        drop(mouse_clicks);

        Inhibit(false)
    }

    // handles the on_scroll event
    fn on_scroll(&mut self, _drawing_area: &DrawingArea, event: &gdk::EventScroll) -> Inhibit {
        let camera_transform: &mut CameraTransform = &mut self.camera_transform.lock().unwrap();
        let scale;

        {
            let keys_pressed = self.window.lock().unwrap().get_keys_pressed();
            let keys_pressed = keys_pressed.lock().unwrap();

            if keys_pressed.is_pressed(&gdk::keys::constants::Shift_L) {
                scale = 1.0;
            } else if keys_pressed.is_pressed(&gdk::keys::constants::Control_L) {
                scale = 0.1;
            } else {
                scale = 0.5;
            }
        }
        

        if event.direction() == ScrollDirection::Up {
            camera_transform.set_scale(camera_transform.get_scale() + scale);
        }

        if event.direction() == ScrollDirection::Down {
            camera_transform.set_scale(camera_transform.get_scale() - scale);
        }

        // let (offset_x, offset_y) = camera_transform.get_offset();
        // self.quilt.lock().unwrap().queue_window_redraw(camera_transform.get_scale(), offset_x, offset_y, drawing_area.get_allocated_width() as f64, drawing_area.get_allocated_height() as f64);
        self.quilt.lock().unwrap().queue_complete_redraw(camera_transform.get_scale());

        *self.needs_updated.lock().unwrap() = true;

        Inhibit(false)
    }

    

    // called on each draw call, will handle any clicks that have happened between frames
    fn handle_clicks(&self, cr: &Context) {
        let mut mouse_clicks = self.mouse_clicks.lock().unwrap();

        //will pass clicks to the items drawn to the screen
        while !mouse_clicks.is_empty() {
            let event = mouse_clicks.pop_front().unwrap();
            
            {
                let mut needs_updated = self.needs_updated.lock().unwrap();

                *needs_updated = *needs_updated || self.quilt.lock().unwrap().click(self, cr, &event);
            }
        }

    }

    pub fn get_window(&self) -> Arc<Mutex<Window>> {
        Arc::clone(&self.window)
    }

    pub fn get_camera_transform(&self) -> Arc<Mutex<CameraTransform>> {
        Arc::clone(&self.camera_transform)
    }

    pub fn get_drawing_area(&self) -> Arc<Mutex<gtk::DrawingArea>> {
        Arc::clone(&self.drawing_area)
    }

    pub fn get_undo_redo(&self) -> Arc<Mutex<UndoRedo<Square>>>{
        Arc::clone(&self.undo_redo)
    }
}

impl KeyListener for Canvas {
    fn on_key_change(&self, keys_pressed: &KeysPressed, _key_changed: Option<(&gdk::EventKey, bool)>) {
        let mut camera_transform = self.camera_transform.lock().unwrap();

        if keys_pressed.is_pressed(&gdk::keys::constants::a) {
            camera_transform.start_move_left();
        } else {
            camera_transform.stop_move_left();
        }

        if keys_pressed.is_pressed(&gdk::keys::constants::d) {
            camera_transform.start_move_right();
        } else {
            camera_transform.stop_move_right();
        }

        if keys_pressed.is_pressed(&gdk::keys::constants::w) {
            camera_transform.start_move_up();
        } else {
            camera_transform.stop_move_up();
        }

        if keys_pressed.is_pressed(&gdk::keys::constants::s) {
            camera_transform.start_move_down();
        } else {
            camera_transform.stop_move_down();
        }

        if (keys_pressed.is_pressed(&gdk::keys::constants::Control_L) || keys_pressed.is_pressed(&gdk::keys::constants::Control_R)) &&
            keys_pressed.is_pressed(&gdk::keys::constants::z) {

            let mut quilt_struct = self.quilt.lock().unwrap();

            let mut undo_redo = self.undo_redo.lock().unwrap();

            if let Some(peek_undo) = undo_redo.peek_undo() {

                let row = peek_undo.row;
                let column = peek_undo.column;

                let undo = undo_redo.undo(quilt_struct.get_square(row, column).unwrap().save_state()).unwrap();

                quilt_struct.set_square(undo.row, undo.column, camera_transform.get_scale(), undo);
            }
        }

        if (keys_pressed.is_pressed(&gdk::keys::constants::Control_L) || keys_pressed.is_pressed(&gdk::keys::constants::Control_R)) &&
            keys_pressed.is_pressed(&gdk::keys::constants::y) {
            let mut quilt_struct = self.quilt.lock().unwrap();

            let mut undo_redo = self.undo_redo.lock().unwrap();

            if let Some(peek_redo) = undo_redo.peek_redo() {

                let row = peek_redo.row;
                let column = peek_redo.column;

                let redo = undo_redo.redo(quilt_struct.get_square(row, column).unwrap().save_state()).unwrap();

                quilt_struct.set_square(redo.row, redo.column, camera_transform.get_scale(), redo);
            }
        }

        if keys_pressed.is_pressed(&gdk::keys::constants::p) {
            let yaml = self.quilt.lock().unwrap().to_save("./saves/test");
            let mut output = String::new();
            let mut emitter = yaml_rust::YamlEmitter::new(&mut output);
            emitter.dump(&yaml).unwrap();

            let path = std::path::Path::new("./saves/test").join("save.yaml");

            let mut file = std::fs::File::create(path).unwrap();

            write!(file, "{}", output).unwrap();
        }

        if keys_pressed.is_pressed(&gdk::keys::constants::o) {
            let path = std::path::Path::new("./saves/test").join("save.yaml");
            let mut file = std::fs::File::open(path).expect("Could not open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Could not read from file");

            let yaml = &yaml_rust::YamlLoader::load_from_str(&contents).unwrap()[0];
            let quilt = *Quilt::from_save(&yaml, "./saves/test");

            *self.quilt.lock().unwrap() = quilt;

            self.quilt.lock().unwrap().queue_complete_redraw(camera_transform.get_scale());
        }
    }
}
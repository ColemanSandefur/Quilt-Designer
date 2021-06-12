use crate::quilt::Quilt;
use crate::frame_timing::FrameTiming;
use crate::click::Click;
use crate::camera_transform::CameraTransform;
use crate::window::Window;

use cairo::Context;
use gdk::{EventMask, ScrollDirection};
use glib::signal::Inhibit;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use gtk::DrawingArea;
use gtk::prelude::*;
use std::ops::Deref;

#[allow(dead_code)]
pub struct Canvas {
    window: Arc<Mutex<Window>>,
    drawing_area: Arc<Mutex<DrawingArea>>,

    mouse_clicks: Arc<Mutex<VecDeque<gdk::EventButton>>>,
    quilt: Arc<Mutex<Quilt>>,
    camera_transform: Arc<Mutex<CameraTransform>>,
    frame_timing: Arc<Mutex<FrameTiming>>,
}

impl Canvas {
    pub fn new(window: Arc<Mutex<Window>>, drawing_area: Arc<Mutex<DrawingArea>>) -> Arc<Mutex<Self>> {
        let quilt = Arc::new(Mutex::new(Quilt::new(5,6)));
        let camera_transform = Arc::new(Mutex::new(CameraTransform::new()));
        let frame_timing = Arc::new(Mutex::new(FrameTiming::new()));
        let mouse_clicks = Arc::new(Mutex::new(VecDeque::with_capacity(10)));

        let s = Arc::new(Mutex::new(Self {
            window: Arc::clone(&window),
            drawing_area: Arc::clone(&drawing_area),

            quilt: Arc::clone(&quilt),
            camera_transform: Arc::clone(&camera_transform),
            frame_timing: Arc::clone(&frame_timing),
            mouse_clicks: Arc::clone(&mouse_clicks),
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

        s
    }

    fn pre_draw(&self, _drawing_area: &DrawingArea, cr: &Context) {
        let frame_timing = self.frame_timing.lock().unwrap();

        //save the context before the transformation
        //just in case we want to remove any transformations later
        cr.save();

        //random offset that I'll remove later
        cr.translate(20.0, 20.0);

        //will handle any necessary camera movements and apply them
        self.handle_camera(cr, frame_timing.deref());
        
        //should handle clicks before draw
        self.handle_clicks(cr);
    }

    fn draw(&self, drawing_area: &DrawingArea, cr: &Context) -> Inhibit {
        self.pre_draw(drawing_area, cr);

        self.quilt.lock().unwrap()
            .draw(cr);

        self.post_draw(drawing_area, cr);

        Inhibit(false)
    }

    fn post_draw(&self, drawing_area: &DrawingArea, cr: &Context) {
        let mut frame_timing = self.frame_timing.lock().unwrap();

        cr.restore();

        //draw frame timing in top left corner
        cr.save();

        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.set_font_size(14.0);
        cr.move_to(4.0, 18.0);
        cr.show_text(&format!("{}ms", frame_timing.delta_frame_time().num_milliseconds()));

        cr.restore();

        //update the last recorded time we rendered a frame
        frame_timing.update_frame_time();
        drawing_area.queue_draw();
    }

    //will add any clicks that drawing area recieves and add it to a queue for handle_clicks to use on next draw
    fn on_click(&self, _drawing_area: &DrawingArea, event: &gdk::EventButton) -> Inhibit {
        let mut mouse_clicks = self.mouse_clicks.lock().unwrap();

        mouse_clicks.push_back(event.to_owned());
        drop(mouse_clicks);

        Inhibit(false)
    }

    //handles the on_scroll event
    fn on_scroll(&self, _drawing_area: &DrawingArea, event: &gdk::EventScroll) -> Inhibit {
        let mut camera_transform = self.camera_transform.lock().unwrap();

        if event.get_direction() == ScrollDirection::Up {
            camera_transform.scale += 0.1;
        }

        if event.get_direction() == ScrollDirection::Down && camera_transform.scale > 0.1 {
            camera_transform.scale -= 0.1;
        }

        Inhibit(false)
    }

    //called by Window (parent)
    pub fn on_key_press(&self, _application_window: &gtk::ApplicationWindow, event: &gdk::EventKey) -> Inhibit {
        let key = event.get_keyval();
        
        if let Some(name) = key.name() {
            let mut camera_transform = self.camera_transform.lock().unwrap();

            // toggles on the pressed keys for camera_transform
            // the camera will actually be moved during the draw call 
            // based off of the time since last frame

            if name.eq("a") {
                camera_transform.start_move_left();
            }

            if name.eq("d") {
                camera_transform.start_move_right();
            }

            if name.eq("w") {
                camera_transform.start_move_up();
            }

            if name.eq("s") {
                camera_transform.start_move_down();
            }
        }

        Inhibit(false)
    }

    //called by Window (parent)
    pub fn on_key_release(&self, _application_window: &gtk::ApplicationWindow, event: &gdk::EventKey) -> Inhibit {
        let key = event.get_keyval();

        if let Some(name) = key.name() {
            let mut camera_transform = self.camera_transform.lock().unwrap();

            // toggles off the pressed keys for camera_transform
            // the camera will actually be moved during the draw call 
            // based off of the time since last frame

            if name.eq("a") {
                camera_transform.stop_move_left();
            }

            if name.eq("d") {
                camera_transform.stop_move_right();
            }

            if name.eq("w") {
                camera_transform.stop_move_up();
            }

            if name.eq("s") {
                camera_transform.stop_move_down();
            }
        }

        Inhibit(false)
    }

    //called on each draw call, will handle any clicks that have happened between frames
    fn handle_clicks(&self, cr: &Context) {
        let mut mouse_clicks = self.mouse_clicks.lock().unwrap();

        //will pass clicks to the items drawn to the screen
        while !mouse_clicks.is_empty() {
            let event = mouse_clicks.pop_front().unwrap();
            self.quilt.lock().unwrap().click(self, cr, &event);
        }
    }

    //called on each draw call, will automatically move the camera and apply any transformations
    fn handle_camera(&self, cr: &Context, frame_timing: &FrameTiming) {
        let mut camera_transform = self.camera_transform.lock().unwrap();

        camera_transform.move_with_keys_pressed(&frame_timing.delta_frame_time());
        camera_transform.apply_transformation(cr);
    }

    pub fn get_window(&self) -> Arc<Mutex<Window>> {
        Arc::clone(&self.window)
    }

    
}
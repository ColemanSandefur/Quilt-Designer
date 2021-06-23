pub mod prefab;

use crate::window::canvas::Canvas;
use crate::util::click::Click;
use crate::texture_brush::TextureBrush;
use crate::path::{Path};

use cairo::{Context};
use gdk::EventButton;
use std::sync::{Arc};

pub struct ChildShape {
    brush: Arc<TextureBrush>,
    paths: Vec<Arc<dyn Path>>
}

impl ChildShape {
    pub fn new() -> Self {
        let brush = Arc::new(TextureBrush::new());
        let paths: Vec<Arc<dyn Path>> = vec![];

        Self {
            brush: brush.clone(),
            paths,
        }
    }

    pub fn new_with_paths(paths: Vec<Arc<dyn Path>>) -> Self {
        let brush = Arc::new(TextureBrush::new());

        Self {
            brush,
            paths,
        }
    }

    fn create_bounds(&self, cr: &Context) {
        for path in &self.paths {
            path.draw_path(cr);
        }
    }

    pub fn draw(&self, cr: &Context) {
        cr.move_to(0.0, 0.0);

        self.create_bounds(cr);
        self.brush.apply(cr);
        
        cr.save();
        {
            cr.save();
            {
                cr.scale(0.99, 0.99);
                self.create_bounds(cr);
                cr.close_path();
            }
            cr.restore();
            
            cr.set_line_width(0.25);
            cr.set_source_rgb(0.0, 0.0, 0.0);
            cr.stroke();
        }

        cr.restore();
    }

    fn change_brush(&mut self, canvas: &Canvas) {
        let brush = canvas.get_window().lock().unwrap().get_brush();
        let brush = brush.lock().unwrap();

        if brush.is_texture_brush() {
            self.brush = brush.get_texture().unwrap().clone()
        } 
    }
}

impl Click for ChildShape {
    fn click(&mut self, canvas: &Canvas, cr: &Context, event: &EventButton) -> bool {
        let (tmp_x, tmp_y) = event.get_position();
        let (x, y) = cr.device_to_user(tmp_x, tmp_y);

        cr.save();
        self.create_bounds(cr);
        let in_bounds = cr.in_fill(x, y);
        cr.restore();

        if  event.get_button() != 1 || !in_bounds {
            return false;
        }

        self.change_brush(canvas);

        true
    }
}

impl Clone for ChildShape {
    fn clone(&self) -> Self {
        let mut paths = Vec::with_capacity(self.paths.len());

        for path in &self.paths {
            paths.push(path.clone_path());
        }

        Self {
            brush: self.brush.clone(),
            // location: self.location.clone(),
            paths,
        }
    }
}
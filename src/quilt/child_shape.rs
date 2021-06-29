use crate::window::canvas::Canvas;
use crate::util::click::Click;
use crate::texture_brush::TextureBrush;
use crate::path::{Path};
use crate::parser::SavableBlueprint;
use crate::path::{Line, Move};

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
        
        cr.save().unwrap();
        {
            cr.save().unwrap();
            {
                cr.scale(0.99, 0.99);
                self.create_bounds(cr);
                cr.close_path();
            }
            cr.restore().unwrap();
            
            cr.set_line_width(0.25);
            cr.set_source_rgb(0.0, 0.0, 0.0);
            cr.stroke().unwrap();
        }

        cr.restore().unwrap();
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
        let (tmp_x, tmp_y) = event.position();
        let (x, y) = cr.device_to_user(tmp_x, tmp_y).unwrap();

        cr.save().unwrap();
        self.create_bounds(cr);
        let in_bounds = cr.in_fill(x, y).unwrap();
        cr.restore().unwrap();

        if  event.button() != 1 || !in_bounds {
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
            paths,
        }
    }
}

impl SavableBlueprint for ChildShape {
    fn from_save_blueprint(yaml_array: &yaml_rust::Yaml) -> Box<Self> {
        let yaml_array = yaml_array.as_vec().unwrap();
        let brush = Arc::new(TextureBrush::new());
        let mut paths: Vec<Arc<dyn Path>> = Vec::with_capacity(yaml_array.len());

        for yaml in yaml_array {
            if let Some(path) = crate::path::from_yaml(yaml) {
                paths.push(path);
            }
        }

        Box::new(Self {
            brush,
            paths,
        })
    }

    fn to_save_blueprint(&self) -> yaml_rust::Yaml {
        let mut yaml = Vec::with_capacity(self.paths.len());

        for path in &self.paths {
            yaml.push(path.to_save_blueprint());
        }

        yaml_rust::Yaml::Array(yaml)
    }
}


pub struct Prefab {}

impl Prefab {
    
    pub fn create_rect(x: f64, y: f64, width: f64, height: f64) -> ChildShape {
        ChildShape::new_with_paths(vec![
            Arc::new(Move::new(x, y)),
            Arc::new(Line::new(x + width, y)),
            Arc::new(Line::new(x + width, y + height)),
            Arc::new(Line::new(x, y + height)),
            Arc::new(Line::new(x, y)),
        ])
    }
    
    pub fn create_triangle(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> ChildShape {
        ChildShape::new_with_paths(vec![
            Arc::new(Move::new(p1.0, p1.1)),
            Arc::new(Line::new(p2.0, p2.1)),
            Arc::new(Line::new(p3.0, p3.1)),
        ])
    }
}
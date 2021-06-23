use crate::window::canvas::Canvas;
use crate::util::click::Click;
use crate::brush::Brush;
use crate::path::{Path, Line, Move};

use cairo::{Context};
use gdk::EventButton;
use std::sync::{Arc, Mutex};

//
// Child shapes
//
// These will be rendered by the square, these are the different patterns that a shape might have
// They save their shape to a surface for easy rendering
//

#[allow(dead_code)]
struct ChildShape {
    brush: Arc<Brush>,
    location: (f64, f64),
    paths: Vec<Arc<dyn Path>>
}

impl ChildShape {
    pub fn new() -> Self {
        let brush = Arc::new(Brush::new());
        let location = (0.0, 0.0);

        let inset = 5.0;
        let paths: Vec<Arc<dyn Path>> = vec![
            Arc::new(Move::new(inset, inset)),
            Arc::new(Line::new(Square::SQUARE_WIDTH - inset, inset)),
            Arc::new(Line::new(Square::SQUARE_WIDTH - inset, Square::SQUARE_WIDTH - inset)),
            Arc::new(Line::new(inset, Square::SQUARE_WIDTH - inset)),
            Arc::new(Line::new(inset, inset))
        ];

        Self {
            brush: brush.clone(),
            location,
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
    }

    fn change_brush(&mut self, canvas: &Canvas) {
        self.brush = canvas.get_window().lock().unwrap()
            .get_brush().lock().unwrap().clone();
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
            location: self.location.clone(),
            paths,
        }
    }
}

pub struct BlockPattern {
    pattern: Vec<ChildShape>
}

impl BlockPattern {
    pub fn new() -> Self {
        let mut pattern = Vec::new();

        pattern.push(ChildShape::new());

        Self {
            pattern
        }
    }

    pub fn draw(&self, cr: &Context) {
        for child in &self.pattern {
            child.draw(cr);
        }
    }
}

impl Click for BlockPattern {
    fn click(&mut self, canvas: &Canvas, cr: &Context, event: &EventButton) -> bool {
        for child in &mut self.pattern {
            if child.click(canvas, cr, event) {
                return true;
            }
        }

        false
    }
}

//
// Square
//

#[derive(Clone)]
pub struct Square {
    brush: Arc<Brush>,
    child_shapes: Arc<Mutex<BlockPattern>>,
}

impl Square {
    pub const SQUARE_WIDTH: f64 = 20.0;
    
    #[allow(dead_code)]
    pub fn new() -> Self {
        let brush = Arc::new(Brush::new());
        let child_shapes = BlockPattern::new();

        Self {
            brush: brush.clone(),
            child_shapes: Arc::new(Mutex::new(child_shapes)),
        }
    }

    pub fn with_brush(brush: Arc<Brush>) -> Self {
        let mut child_shapes = Vec::new();

        child_shapes.push(ChildShape::new());
        let child_shapes = BlockPattern::new();

        Self {
            brush: brush.clone(),
            child_shapes: Arc::new(Mutex::new(child_shapes)),
        }
    }

    pub fn draw(&self, cr: &Context) {

        cr.save();

        let line_width = 0.25;
        
        cr.save();

        // clip the region to just be the square (prevent 'over spray')
        cr.move_to(0.0, 0.0);
        cr.line_to(Square::SQUARE_WIDTH, 0.0);
        cr.line_to(Square::SQUARE_WIDTH, Square::SQUARE_WIDTH);
        cr.line_to(0.0, Square::SQUARE_WIDTH);
        cr.line_to(0.0, 0.0);
        cr.line_to(Square::SQUARE_WIDTH, 0.0);

        cr.clip();

        // draw background of square
        cr.move_to(0.0, 0.0);
        cr.line_to(Square::SQUARE_WIDTH, 0.0);
        cr.line_to(Square::SQUARE_WIDTH, Square::SQUARE_WIDTH);
        cr.line_to(0.0, Square::SQUARE_WIDTH );
        cr.line_to(0.0, 0.0);
        self.brush.apply(cr);

        self.child_shapes.lock().unwrap().draw(cr);

        cr.restore();

        cr.set_line_width(line_width * 2.0);
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.move_to(0.0, 0.0);
        cr.line_to(Square::SQUARE_WIDTH, 0.0);
        cr.line_to(Square::SQUARE_WIDTH, Square::SQUARE_WIDTH);
        cr.line_to(0.0, Square::SQUARE_WIDTH);
        cr.line_to(0.0, 0.0);
        cr.line_to(Square::SQUARE_WIDTH, 0.0);
        cr.stroke();

        cr.restore();
    }

    // sets the square's brush to the same one that Window has
    fn change_brush(&mut self, canvas: &Canvas) {
        self.brush = canvas.get_window().lock().unwrap()
            .get_brush().lock().unwrap().clone();
    }
}

impl Click for Square {
    fn click(&mut self, canvas: &Canvas, cr: &Context, event: &EventButton) -> bool {
        let (tmp_x, tmp_y) = event.get_position();
        let (x, y) = cr.device_to_user(tmp_x, tmp_y);

        if  (event.get_button() != 1) ||
            (x < 0.0 || x >= Square::SQUARE_WIDTH) ||
            (y < 0.0 || y >= Square::SQUARE_WIDTH)
        {
            return false;
        }

        if self.child_shapes.lock().unwrap().click(canvas, cr, event) {
            return true
        }

        self.change_brush(canvas);

        true
    }
}
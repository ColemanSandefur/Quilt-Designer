
use crate::quilt::child_shape::ChildShape;
use crate::window::canvas::Canvas;
use crate::util::click::Click;
use crate::texture_brush::TextureBrush;

use cairo::{Context};
use gdk::EventButton;
use std::sync::{Arc, Mutex};

//
// Child shapes
//
// These will be rendered by the square, these are the different patterns that a shape might have
// They save their shape to a surface for easy rendering
//


#[derive(Clone)]
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

    pub fn new_pattern(pattern: Vec<ChildShape>) -> Self {
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
    brush: Arc<TextureBrush>,
    block_pattern: Arc<Mutex<BlockPattern>>,
}

impl Square {
    pub const SQUARE_WIDTH: f64 = 20.0;
    
    #[allow(dead_code)]
    pub fn new() -> Self {
        let brush = Arc::new(TextureBrush::new());
        let block_pattern = BlockPattern::new();

        Self {
            brush: brush.clone(),
            block_pattern: Arc::new(Mutex::new(block_pattern)),
        }
    }

    pub fn with_brush(brush: Arc<TextureBrush>) -> Self {
        let block_pattern = BlockPattern::new();

        Self {
            brush: brush.clone(),
            block_pattern: Arc::new(Mutex::new(block_pattern)),
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

        self.block_pattern.lock().unwrap().draw(cr);

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
        let brush = canvas.get_window().lock().unwrap().get_brush();
        let brush = brush.lock().unwrap();

        if brush.is_texture_brush() {
            self.brush = brush.get_texture().unwrap().clone()
        }
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

        {
            let brush = canvas.get_window().lock().unwrap().get_brush();
            let brush = brush.lock().unwrap();

            if brush.is_block_pattern_brush() {
                self.block_pattern = Arc::new(Mutex::new(brush.get_block_pattern().unwrap().clone()));

                return true
            }
        }

        if self.block_pattern.lock().unwrap().click(canvas, cr, event) {
            return true
        }

        self.change_brush(canvas);

        true
    }
}


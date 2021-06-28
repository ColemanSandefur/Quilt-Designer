
use crate::quilt::child_shape::ChildShape;
use crate::window::canvas::Canvas;
use crate::util::click::Click;
use crate::texture_brush::TextureBrush;
use crate::parser::SavableBlueprint;

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
    rotation: f64,
    pattern: Vec<ChildShape>,
}

impl BlockPattern {
    pub fn new() -> Self {
        let mut pattern = Vec::new();

        pattern.push(ChildShape::new());

        Self {
            pattern,
            rotation: 0.0,
        }
    }

    pub fn new_pattern(pattern: Vec<ChildShape>) -> Self {
        Self {
            pattern,
            rotation: 0.0,
        }
    }

    pub fn apply_transformation(&self, cr: &Context) {
        cr.translate(Square::SQUARE_WIDTH / 2.0, Square::SQUARE_WIDTH / 2.0);
        cr.rotate(self.rotation);
        cr.translate(-Square::SQUARE_WIDTH / 2.0, -Square::SQUARE_WIDTH / 2.0);
    }

    pub fn draw(&self, cr: &Context) {

        self.apply_transformation(cr);

        for child in &self.pattern {
            child.draw(cr);
        }
    }

    pub fn rotate(&mut self, amount_radians: f64) {
        let rot = self.rotation;
        self.rotation = rot + amount_radians;
    }
}

impl Click for BlockPattern {
    fn click(&mut self, canvas: &Canvas, cr: &Context, event: &EventButton) -> bool {

        self.apply_transformation(cr);

        for child in &mut self.pattern {
            if child.click(canvas, cr, event) {
                return true;
            }
        }

        false
    }
}

impl SavableBlueprint for BlockPattern {
    fn from_save_blueprint(yaml_array: &yaml_rust::Yaml) -> Box<Self> {
        let yaml_array = yaml_array.as_vec().unwrap();
        let mut pattern = Vec::with_capacity(yaml_array.len());

        for yaml in yaml_array {
            pattern.push(*ChildShape::from_save_blueprint(yaml));
        }

        Box::new(Self {
            pattern,
            rotation: 0.0,
        })
    }

    fn to_save_blueprint(&self) -> yaml_rust::Yaml {
        let mut yaml = Vec::with_capacity(self.pattern.len());

        for shape in &self.pattern {
            yaml.push(shape.to_save_blueprint());
        }

        yaml_rust::Yaml::Array(yaml)
    }
}

//
// Square
//

#[derive(Clone)]
pub struct Square {
    pub row: usize,
    pub column: usize,
    brush: Arc<TextureBrush>,
    block_pattern: Arc<Mutex<BlockPattern>>,
}

impl Square {
    pub const SQUARE_WIDTH: f64 = 20.0;
    
    #[allow(dead_code)]
    pub fn new(row: usize, column: usize) -> Self {
        let brush = Arc::new(TextureBrush::new());
        let block_pattern = BlockPattern::new();

        Self {
            row,
            column,
            brush: brush.clone(),
            block_pattern: Arc::new(Mutex::new(block_pattern)),
        }
    }

    pub fn with_brush(row: usize, column: usize, brush: Arc<TextureBrush>) -> Self {
        let block_pattern = BlockPattern::new();

        Self {
            row,
            column,
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

    // use if you want to keep everything, else clone will just clone the Arc references
    pub fn save_state(& self) -> Self {
        let block_pattern_mutex = self.block_pattern.lock().unwrap();

        Self {
            row: self.row,
            column: self.column,
            brush: self.brush.clone(),
            block_pattern: Arc::new(Mutex::new(block_pattern_mutex.clone()))
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

        canvas.get_undo_redo().lock().unwrap().changed(self.save_state());

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

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "row: {}, column: {}, brush: {}", self.row, self.column, self.brush)
    }
}
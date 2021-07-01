use crate::draw::texture_brush::TextureBrush;
use crate::quilt::block_pattern::BlockPattern;
use crate::util::click::Click;
use crate::util::parser::*;
use crate::window::canvas::Canvas;

use cairo::{Context};
use gdk::EventButton;
use std::sync::{Arc, Mutex};
use yaml_rust::Yaml;

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

        cr.save().unwrap();

        let line_width = 0.25;
        
        cr.save().unwrap();

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

        cr.restore().unwrap();

        cr.set_line_width(line_width * 2.0);
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.move_to(0.0, 0.0);
        cr.line_to(Square::SQUARE_WIDTH, 0.0);
        cr.line_to(Square::SQUARE_WIDTH, Square::SQUARE_WIDTH);
        cr.line_to(0.0, Square::SQUARE_WIDTH);
        cr.line_to(0.0, 0.0);
        cr.line_to(Square::SQUARE_WIDTH, 0.0);
        cr.stroke().unwrap();

        cr.restore().unwrap();
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
        let (tmp_x, tmp_y) = event.position();
        let (x, y) = cr.device_to_user(tmp_x, tmp_y).unwrap();

        if  (event.button() != 1) ||
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

impl Savable for Square {
    fn to_save(&self, save_path: &mut SaveData) -> Yaml {
        Serializer::create_map(vec!{
            ("row", Serializer::serialize(self.row as i64)),
            ("column", Serializer::serialize(self.column as i64)),
            ("brush", self.brush.to_save(save_path)),
            ("block_pattern", self.block_pattern.lock().unwrap().to_save(save_path)),
        })
    }

    fn from_save(yaml: &Yaml, save_path: &mut SaveData) -> Box<Self> {
        let map = Parser::to_map(yaml);

        let row: i64 = Parser::parse(map.get(&Serializer::serialize("row")).unwrap());
        let column: i64 = Parser::parse(map.get(&Serializer::serialize("column")).unwrap());
        let brush = *TextureBrush::from_save(map.get(&Serializer::serialize("brush")).unwrap(), save_path);
        let block_pattern = *BlockPattern::from_save(map.get(&Serializer::serialize("block_pattern")).unwrap(), save_path);

        Box::new(Self {
            row: row as usize,
            column: column as usize,
            brush: Arc::new(brush),
            block_pattern: Arc::new(Mutex::new(block_pattern))
        })
    }
}
use crate::canvas::Canvas;
use crate::click::Click;
use crate::brush::Brush;
use cairo::{Context};
use gdk::EventButton;


static SQUARE_WIDTH: f64 = 20.0;

struct Square {
    brush: Brush
}

impl Square {
    pub fn new() -> Self {
        Self {
            brush: Brush::new_color((0.0, 1.0, 0.0))
        }
    }

    pub fn draw(&self, cr: &Context) {
        cr.save();

        let line_width = 0.5;

        cr.move_to(0.0 + line_width, 0.0 + line_width);
        cr.line_to(SQUARE_WIDTH - line_width, 0.0 + line_width);
        cr.line_to(SQUARE_WIDTH - line_width, SQUARE_WIDTH - line_width);
        cr.line_to(0.0 + line_width, SQUARE_WIDTH - line_width);
        cr.line_to(0.0 + line_width, 0.0 + line_width);
        self.brush.apply(cr);
        cr.set_line_width(1.0);
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.move_to(0.0, 0.0);
        cr.line_to(SQUARE_WIDTH, 0.0);
        cr.line_to(SQUARE_WIDTH, SQUARE_WIDTH);
        cr.line_to(0.0, SQUARE_WIDTH);
        cr.line_to(0.0, 0.0);
        cr.stroke();

        cr.restore();
    }
}

impl Click for Square {
    fn click(&mut self, window: &Canvas, cr: &Context, event: &EventButton) -> bool {
        // let (tmp_x, tmp_y) = event.get_position();
        let (tmp_x, tmp_y) = event.get_position();
        let (x, y) = cr.device_to_user(tmp_x, tmp_y);

        if x < 0.0 || x >= SQUARE_WIDTH {
            return false;
        }

        if y < 0.0 || y >= SQUARE_WIDTH {
            return false;
        }

        self.brush = window.get_window().lock().unwrap()
            .get_brush().lock().unwrap().clone();

        true
    }
}

pub struct Quilt {
    width: usize,
    height: usize,
    quilt: Vec<Vec<Square>>
}

impl Quilt {
    pub fn new(width: usize, height: usize) -> Self {

        let mut quilt: Vec<Vec<Square>> = Vec::new();

        for _ in 0..height {
            let mut row = Vec::new();

            for _ in 0..width {
                row.push(Square::new());
            }

            quilt.push(row);
        }

        Quilt {
            width: width,
            height: height,
            quilt: quilt
        }
    }

    pub fn draw(&self, cr: &Context) {
        cr.save();

        for row in 0..self.height {
            
            for col in 0..self.width {
                self.quilt[row][col].draw(cr);
                cr.translate(SQUARE_WIDTH, 0.0);
            }

            cr.translate(SQUARE_WIDTH * -(self.width as f64), SQUARE_WIDTH);

        }

        cr.restore();
    }
}

impl Click for Quilt {
    fn click(&mut self, window: &Canvas, cr: &Context, event: &EventButton) -> bool {
        // let (tmp_x, tmp_y) = event.get_position();
        let (tmp_x, tmp_y) = event.get_position();
        let (x, y) = cr.device_to_user(tmp_x, tmp_y);

        if x < 0.0 || x >= self.width as f64 * SQUARE_WIDTH {
            return false;
        }

        if y < 0.0 || y >= self.height as f64 * SQUARE_WIDTH {
            return false;
        }

        cr.save();

        for row in 0..self.height {
            cr.save();

            for col in 0..self.width {
                self.quilt[row][col].click(window, cr, event);
                cr.translate(SQUARE_WIDTH, 0.0);
            }

            cr.restore();

            cr.translate(0.0, SQUARE_WIDTH);

        }

        cr.restore();

        true
    }
}
use crate::click::Click;
use cairo::{Context};
use gdk::EventButton;


static SQUARE_WIDTH: f64 = 20.0;

struct Square {
    color: (f64, f64, f64)
}

impl Square {
    pub fn new() -> Self {
        Self {
            color: (01.0, 0.0, 0.0)
        }
    }

    pub fn draw(&self, cr: &Context) {
        cr.save();

        cr.set_source_rgb(self.color.0, self.color.1, self.color.2);
        cr.move_to(0.0, 0.0);
        cr.line_to(SQUARE_WIDTH, 0.0);
        cr.line_to(SQUARE_WIDTH, SQUARE_WIDTH);
        cr.line_to(0.0, SQUARE_WIDTH);
        cr.fill();
        cr.set_line_width(1.0);
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.move_to(0.0, 0.0);
        cr.line_to(SQUARE_WIDTH, 0.0);
        cr.line_to(SQUARE_WIDTH, SQUARE_WIDTH);
        cr.line_to(0.0, SQUARE_WIDTH);
        cr.stroke();

        cr.restore();
    }
}

impl Click for Square {
    fn click(&mut self, cr: &Context, event: &EventButton) -> bool {
        // let (tmp_x, tmp_y) = event.get_position();
        let (tmp_x, tmp_y) = event.get_position();
        let (x, y) = cr.device_to_user(tmp_x, tmp_y);

        if x < 0.0 || x >= SQUARE_WIDTH {
            return false;
        }

        if y < 0.0 || y >= SQUARE_WIDTH {
            return false;
        }

        self.color = (0.0, 1.0, 0.0);

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
    fn click(&mut self, cr: &Context, event: &EventButton) -> bool {
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
                self.quilt[row][col].click(cr, event);
                cr.translate(SQUARE_WIDTH, 0.0);
            }

            cr.restore();

            cr.translate(0.0, SQUARE_WIDTH);

        }

        cr.restore();

        true
    }
}
use cairo::{Context};


static SQUARE_WIDTH: f64 = 20.0;

struct Square {
}

impl Square {
    pub fn draw(&self, cr: &Context) {
        cr.save();

        cr.set_source_rgb(1.0, 0.0, 0.0);
        cr.line_to(0.0, 0.0);
        cr.line_to(SQUARE_WIDTH, 0.0);
        cr.line_to(SQUARE_WIDTH, SQUARE_WIDTH);
        cr.line_to(0.0, SQUARE_WIDTH);
        cr.fill();
        cr.set_line_width(1.0);
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.line_to(0.0, 0.0);
        cr.line_to(SQUARE_WIDTH, 0.0);
        cr.line_to(SQUARE_WIDTH, SQUARE_WIDTH);
        cr.line_to(0.0, SQUARE_WIDTH);
        cr.stroke();

        cr.restore();
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
                row.push(Square {

                });
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
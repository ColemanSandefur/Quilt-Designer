use crate::window::canvas::Canvas;
use crate::click::Click;
use crate::brush::Brush;

use cairo::{Context};
use gdk::EventButton;
use std::sync::Arc;

//
// The main thing that I will be drawing is the quilt
//
// The quilt is made up of a 2d array of squares
// Squares will have different colors and shapes that will be filled
// The main Window's brush will be used when changing a square's color:
//   Brushes are immutable so when the user chooses a new brush the reference to the "main brush" changes
//   Each square holds a reference to their paint brush and uses that to paint each frame
//   These are atomic references so they will destruct when no pointers are left 
//

static SQUARE_WIDTH: f64 = 20.0;

struct Square {
    brush: Arc<Brush>
}

impl Square {
    pub fn new() -> Self {
        Self {
            brush: Arc::new(Brush::new_color((0.0, 1.0, 0.0)))
        }
    }

    pub fn draw(&self, cr: &Context) {
        cr.save();

        let line_width = 0.25;

        cr.move_to(0.0 + line_width, 0.0 + line_width);
        cr.line_to(SQUARE_WIDTH - line_width, 0.0 + line_width);
        cr.line_to(SQUARE_WIDTH - line_width, SQUARE_WIDTH - line_width);
        cr.line_to(0.0 + line_width, SQUARE_WIDTH - line_width);
        cr.line_to(0.0 + line_width, 0.0 + line_width);
        self.brush.apply(cr);
        cr.set_line_width(line_width * 2.0);
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.move_to(0.0, 0.0);
        cr.line_to(SQUARE_WIDTH, 0.0);
        cr.line_to(SQUARE_WIDTH, SQUARE_WIDTH);
        cr.line_to(0.0, SQUARE_WIDTH);
        cr.line_to(0.0, 0.0);
        cr.line_to(SQUARE_WIDTH, 0.0);
        cr.stroke();

        cr.restore();
    }
}

impl Click for Square {
    fn click(&mut self, canvas: &Canvas, cr: &Context, event: &EventButton) -> bool {
        let (tmp_x, tmp_y) = event.get_position();
        let (x, y) = cr.device_to_user(tmp_x, tmp_y);

        if  (event.get_button() != 1) ||
            (x < 0.0 || x >= SQUARE_WIDTH) ||
            (y < 0.0 || y >= SQUARE_WIDTH)
        {
            return false;
        }

        self.brush = canvas.get_window().lock().unwrap()
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
        cr.save();

        // a rather jank solution to click registration
        // the left side bar messes up my original way to convert click position into canvas space,
        // easy solution is to revert matrix back to default and then apply the camera transformations again
        // this means that we are technically rendering the quilt in a different place than our click registration

        cr.identity_matrix(); // remove all transformations
        window.get_camera_transform().lock().unwrap().apply_transformation(cr); // re-apply the camera transformations
        let (tmp_x, tmp_y) = event.get_position();
        let (x, y) = cr.device_to_user(tmp_x, tmp_y); // calculate position

        let mut result: bool = false;

        if  x < 0.0 || x >= self.width  as f64 * SQUARE_WIDTH ||
            y < 0.0 || y >= self.height as f64 * SQUARE_WIDTH  {
            
            result = false;

        } else {

            cr.save();

            for row in 0..self.height {

                cr.save();

                for col in 0..self.width {
                    result = result || self.quilt[row][col].click(window, cr, event);
                    cr.translate(SQUARE_WIDTH, 0.0);
                }

                cr.restore();

                cr.translate(0.0, SQUARE_WIDTH);

            }

            cr.restore();
            
        }
        
        cr.restore();

        result
    }
}
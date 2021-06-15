use crate::window::canvas::Canvas;
use crate::util::click::Click;
use crate::brush::Brush;

use cairo::{Context};
use gdk::EventButton;
use std::sync::Arc;
use std::f64::consts::PI;
use gdk::prelude::*;
use std::*;
use gdk_pixbuf::Pixbuf;

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

pub static SQUARE_WIDTH: f64 = 20.0;

struct Square {
    brush: Arc<Brush>,
    image: Option<Arc<Pixbuf>>,
    scale: f64,
}

impl Square {

    fn load_image(name: &str) -> Option<Arc<Pixbuf>> {
        match Pixbuf::from_file(name) {
            Ok(buf) => Some(Arc::new(buf)),
            Err(err) => {
                println!("{:?}", err);
                None
            },
        }
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        let image = Square::load_image("./test_image.png");
        
        let scale = match &image {
            Some(pixbuf) => {
                let small_side = std::cmp::min(pixbuf.get_width(), pixbuf.get_height());
                SQUARE_WIDTH / small_side as f64
            },
            _ => 0.0
        };

        Self {
            brush: Arc::new(Brush::new_color((0.0, 1.0, 0.0))),
            image: image,
            scale: scale,
        }
    }

    pub fn with_brush(brush: Arc<Brush>) -> Self {
        let image = Square::load_image("./test_image.png");
        
        let scale = match &image {
            Some(pixbuf) => {
                let small_side = std::cmp::min(pixbuf.get_width(), pixbuf.get_height());
                SQUARE_WIDTH / small_side as f64
            },
            _ => 0.0
        };

        Self {
            brush: brush.clone(),
            image: image,
            scale: scale,
        }
    }

    pub fn draw(&self, cr: &Context) {
        cr.save();

        let line_width = 0.25;
        
        cr.save();

        cr.move_to(0.0, 0.0);
        cr.line_to(SQUARE_WIDTH, 0.0);
        cr.line_to(SQUARE_WIDTH, SQUARE_WIDTH);
        cr.line_to(0.0, SQUARE_WIDTH);
        cr.line_to(0.0, 0.0);
        cr.line_to(SQUARE_WIDTH, 0.0);

        cr.clip();

        cr.move_to(0.0 + line_width, 0.0 + line_width);
        cr.line_to(SQUARE_WIDTH - line_width, 0.0 + line_width);
        cr.line_to(SQUARE_WIDTH - line_width, SQUARE_WIDTH - line_width);
        cr.line_to(0.0 + line_width, SQUARE_WIDTH - line_width);
        cr.line_to(0.0 + line_width, 0.0 + line_width);
        self.brush.apply(cr);

        
        if let Some(pixbuf) = &self.image {
            cr.save();
            cr.move_to(SQUARE_WIDTH, SQUARE_WIDTH/2.0);
            cr.arc(SQUARE_WIDTH/2.0, SQUARE_WIDTH/2.0, SQUARE_WIDTH/2.0, 0.0, PI);
            cr.line_to(SQUARE_WIDTH, SQUARE_WIDTH/2.0);

            cr.clip();
            
            cr.scale(self.scale, self.scale);

            cr.set_source_pixbuf(pixbuf, 0.0, 0.0);
            cr.paint();

            cr.restore();
        }

        cr.restore();

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
            (x < 0.0 || x >= SQUARE_WIDTH) ||
            (y < 0.0 || y >= SQUARE_WIDTH)
        {
            return false;
        }

        self.change_brush(canvas);

        true
    }
}

pub struct Quilt {
    pub width: usize,
    pub height: usize,
    quilt: Vec<Vec<Square>>,
}

impl Quilt {
    pub fn new(width: usize, height: usize) -> Self {

        let mut quilt: Vec<Vec<Square>> = Vec::new();
        let brush = Arc::new(Brush::new_color((1.0, 1.0, 0.0)));

        for _ in 0..height {
            let mut row = Vec::new();

            for _ in 0..width {
                row.push(Square::with_brush(brush.clone()));
            }

            quilt.push(row);
        }

        Quilt {
            width: width,
            height: height,
            quilt: quilt,
        }
    }

    pub fn draw(&mut self, cr: &Context) {
        cr.save();

        cr.move_to(0.0, 0.0);
        cr.line_to(SQUARE_WIDTH * self.width as f64, 0.0);
        cr.line_to(SQUARE_WIDTH * self.width as f64, SQUARE_WIDTH * self.height as f64);
        cr.line_to(0.0, SQUARE_WIDTH * self.height as f64);
        cr.line_to(0.0, 0.0);

        cr.clip();

        for row in 0..self.height {
                        
            for col in 0..self.width {
                self.quilt[row][col].draw(cr);
                cr.translate(SQUARE_WIDTH, 0.0);
            }

            cr.translate(SQUARE_WIDTH * -(self.width as f64), SQUARE_WIDTH);

        }

        cr.restore();

    }

    pub fn get_dimensions_pixel(&self) -> (f64, f64) {
        return (
            SQUARE_WIDTH * self.width as f64, 
            SQUARE_WIDTH * self.height as f64
        )
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
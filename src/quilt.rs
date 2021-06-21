use crate::window::canvas::Canvas;
use crate::util::click::Click;
use crate::util::image::Image;
use crate::brush::{Brush, Texture};
use crate::camera_transform::CameraTransform;

use cairo::{Context};
use gdk::EventButton;
use std::sync::{Arc, Mutex};
use std::*;

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

//
// Child shapes
//
// These will be rendered by the square, these are the different patterns that a shape might have
// They save their shape to a surface for easy rendering
//

#[allow(dead_code)]
struct ChildShape {
    brush: Arc<Brush>,
    scale: f64,
}

impl ChildShape {

}

//
// Square
//

#[derive(Clone)]
struct Square {
    brush: Arc<Brush>,
}

impl Square {

    #[allow(dead_code)]
    pub fn new() -> Self {
        let brush = Arc::new(Brush::new());

        Self {
            brush: brush.clone(),
        }
    }

    pub fn with_brush(brush: Arc<Brush>) -> Self {
        Self {
            brush: brush.clone(),
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

//
// Quilt
//

pub struct Quilt {
    pub width: usize,
    pub height: usize,
    quilt: Vec<Vec<Square>>,
}

impl Quilt {

    pub fn new(width: usize, height: usize) -> Self {

        let mut quilt: Vec<Vec<Square>> = Vec::new();
        let brush = match Texture::new("./test_image.jpg") {
            Ok(texture) => {
                println!("Texture found!");
                Arc::new(Brush::new_texture(texture))
            },
            Err(err) => {
                println!("{:?}", err);
                Arc::new(Brush::new_color((1.0, 1.0, 0.0)))
            }
        };

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

    const NUM_THREADS: usize = 2;

    fn start_thread(squares: Arc<Mutex<Vec<(Square, u32)>>>, width: i32, zoom: f64) -> std::thread::JoinHandle<Vec<(Image, u32)>> {
        let mut images: Vec<(Image, u32)> = Vec::with_capacity(squares.lock().unwrap().len());

        thread::spawn(move || {
            
            // let image = Image::new(width, width);
            
            let squares = squares.lock().unwrap();

            for (square, original_index) in squares.iter() {
                let mut image = Image::new(width, width);

                image.with_surface(|surface| {
                    let cr = cairo::Context::new(&surface);

                    cr.scale(zoom, zoom);

                    square.draw(&cr);
                });

                images.push((image, *original_index));
            }

            images

        })
    }

    pub fn draw(this: Arc<Mutex<Self>>, cr: &Context, camera_transform: Arc<Mutex<CameraTransform>>) {
        cr.save();

        let scale = camera_transform.lock().unwrap().get_scale();

        {
            let this = this.lock().unwrap();
            cr.move_to(0.0, 0.0);
            cr.line_to(SQUARE_WIDTH * this.width as f64, 0.0);
            cr.line_to(SQUARE_WIDTH * this.width as f64, SQUARE_WIDTH * this.height as f64);
            cr.line_to(0.0, SQUARE_WIDTH * this.height as f64);
            cr.line_to(0.0, 0.0);
        }

        cr.clip();

        let mut all_threads = Vec::with_capacity(Quilt::NUM_THREADS);

        let mut all_squares;
        {
            let this = this.lock().unwrap(); 
            all_squares = Vec::with_capacity(Quilt::NUM_THREADS);

            for _ in 0..Quilt::NUM_THREADS {
                all_squares.push(Arc::new(Mutex::new(Vec::with_capacity((this.width * this.height) / Quilt::NUM_THREADS + 1))));
            }

            let mut thread_number = 0;
            let mut index = 0;
            for row in &this.quilt {
                for square in row {
                    thread_number = thread_number % Quilt::NUM_THREADS;

                    all_squares[thread_number].lock().unwrap().push((square.clone(), index));

                    thread_number += 1;
                    index += 1;
                }
            }
        }


        for thread_num in 0..Quilt::NUM_THREADS {
            all_threads.push(Quilt::start_thread(all_squares[thread_num].clone(), (scale * SQUARE_WIDTH) as i32, scale));
        }

        let width = this.lock().unwrap().width;

        cr.save();
        cr.scale(1.0/scale, 1.0/scale);
        for thread in all_threads {
            let res = thread.join().unwrap();

            for (mut image, index) in res {

                let row = index as usize / width;
                let column = index as usize % width;
                

                image.with_surface(|surface| {
                    cr.set_source_surface(surface, SQUARE_WIDTH * column as f64 * scale, SQUARE_WIDTH * row as f64 * scale);

                    cr.paint();

                    cr.set_source_rgb(0.0, 0.0, 0.0);
                });

            }

        }
        cr.restore();

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
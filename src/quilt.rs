use crate::window::canvas::Canvas;
use crate::util::click::Click;
use crate::util::image::Image;
use crate::brush::{Brush, Texture};
use crate::camera_transform::CameraTransform;

use cairo::{Context};
use gdk::EventButton;
use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

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
// the images property is a cache of previous renderings, to update the cache you must queue an update
// the update will then be automatically put on a dedicated thread to render, it then gets sent to the ready_rx thread
// that thread will then update the cache and in the next draw call it will be rendered
//

#[allow(dead_code)]
pub struct Quilt {
    pub width: usize,
    pub height: usize,
    quilt: Vec<Vec<Square>>,
    images: Arc<Mutex<Vec<(Image, f64)>>>, //pairs image with the scale it was rendered at
    
    
    //for multi threading
    ready_tx: glib::Sender<((Image, f64), usize)>, // sending rendered images to be displayed on screen
    
    thread_streams: Vec<mpsc::Sender<(Square, f64, usize)>>, // sending squares to be rendered
    next_thread_index: u8, // manages which thread will be used next
}

impl Quilt {
    const NUM_THREADS: usize = 2;

    pub fn new(width: usize, height: usize) -> Self {

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
        
        let mut quilt: Vec<Vec<Square>> = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::new();
            
            for _ in 0..width {
                row.push(Square::with_brush(brush.clone()));
            }
            
            quilt.push(row);
        }
        
        let mut images = Vec::with_capacity(width * height);
        for index in 0..width*height {


            let mut image = Image::new(SQUARE_WIDTH as i32, SQUARE_WIDTH as i32);

            image.with_surface(|surface| {
                let cr = cairo::Context::new(&surface);

                let row = index as usize / width;
                let column = index as usize % width;

                quilt[row][column].draw(&cr);
            });

            images.push((image, 1.0));

        }

        let images = Arc::new(Mutex::new(images));

        let (ready_tx, ready_rx) = glib::MainContext::channel::<((Image, f64), usize)>(glib::PRIORITY_DEFAULT); //image, zoom, index

        let images_clone = images.clone();
        ready_rx.attach(None, move |(data, index)| {
            let images = images_clone.clone();

            images.lock().unwrap()[index] = data;

            glib::Continue(true)
        });

        let mut thread_streams = Vec::with_capacity(Quilt::NUM_THREADS);
        for _thread_num in 0..Quilt::NUM_THREADS {
            let (tx, rx) = mpsc::channel::<(Square, f64, usize)>();

            Quilt::start_render_thread(rx, ready_tx.clone());

            thread_streams.push(tx);
        }

        Quilt {
            width: width,
            height: height,
            quilt: quilt,
            images: images,

            ready_tx: ready_tx,
            thread_streams: thread_streams,
            next_thread_index: 0,
        }
    }


    fn queue_draw(&mut self, index: usize, scale: f64) {
        self.next_thread_index = self.next_thread_index % Quilt::NUM_THREADS as u8;

        let transmitter = &self.thread_streams[self.next_thread_index as usize];
        let row = index as usize / self.width;
        let column = index as usize % self.width;
        let square = &self.quilt[row][column];

        let _ = transmitter.send((square.clone(), scale, index));

        self.next_thread_index += 1;
    }

    pub fn queue_complete_redraw(&mut self, scale: f64) {
        let num_squares = self.width * self.height;

        for i in 0..num_squares {
            self.queue_draw(i, scale);
        }
    }

    fn start_render_thread(rx: mpsc::Receiver<(Square, f64, usize)>, ready_tx: glib::Sender<((Image, f64), usize)>) {
        thread::spawn(move || {
            for (square, scale, index) in rx.iter() {
                let mut image = Image::new((SQUARE_WIDTH * scale) as i32, (SQUARE_WIDTH * scale) as i32);
                image.with_surface(|surface| {
                    let cr = cairo::Context::new(&surface);

                    cr.scale(scale, scale);

                    square.draw(&cr);
                });
                // self.images[index] = (image, scale);
                let _ = ready_tx.send(((image, scale), index));
            }
        });
    }

    pub fn draw(&mut self, cr: &Context, _camera_transform: Arc<Mutex<CameraTransform>>) {
        cr.save();

        {
            cr.move_to(0.0, 0.0);
            cr.line_to(SQUARE_WIDTH * self.width as f64, 0.0);
            cr.line_to(SQUARE_WIDTH * self.width as f64, SQUARE_WIDTH * self.height as f64);
            cr.line_to(0.0, SQUARE_WIDTH * self.height as f64);
            cr.line_to(0.0, 0.0);
        }

        cr.clip();

        let width = self.width;
        
        let mut images = self.images.lock().unwrap();

        for index in 0..images.len() {
            let (image, scale) = &mut images[index];

            let scale = *scale;
            cr.save();
            cr.scale(1.0/scale, 1.0/scale);

            image.borrow_mut().with_surface(|surface| {

                let row = index as usize / width;
                let column = index as usize % width;

                cr.set_source_surface(surface, SQUARE_WIDTH * column as f64 * scale, SQUARE_WIDTH * row as f64 * scale);

                cr.paint();

                cr.set_source_rgb(0.0, 0.0, 0.0);
            });
            cr.restore();
        }

        cr.restore()

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
                    result = result || match self.quilt[row][col].click(window, cr, event) {
                        true => {
                            self.queue_draw(row * self.width + col, window.get_camera_transform().lock().unwrap().get_scale());

                            true
                        },
                        false => false
                    };
                    
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
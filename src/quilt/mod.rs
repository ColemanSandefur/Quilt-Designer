pub mod square;
pub mod child_shape;

use square::Square;
use crate::window::canvas::Canvas;
use crate::util::click::Click;
use crate::util::image::Image;
use crate::util::rectangle::Rectangle;
use crate::texture_brush::{TextureBrush};
use crate::camera_transform::CameraTransform;
use crate::util::undo_redo::UndoRedo;

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
    undo_redo: UndoRedo<Square>,
    
    
    //for multi threading
    ready_tx: glib::Sender<((Image, f64), usize)>, // sending rendered images to be displayed on screen
    
    thread_streams: Vec<mpsc::Sender<(Square, f64, usize, Arc<Mutex<(bool, bool)>>)>>, // sending squares to be rendered
    next_thread_index: u8, // manages which thread will be used next
    queued_squares: Box<[Arc<Mutex<(bool, bool)>>]>, // should draw, force draw
}

impl Quilt {
    const NUM_THREADS: usize = 4;
    const OVER_SCALE: f64 = 2.0; // added to actual scale when rendering should make zooming in less jarring

    pub fn new(width: usize, height: usize) -> Self {

        let brush = Arc::new(TextureBrush::new_color((1.0, 1.0, 1.0)));
        
        let mut quilt: Vec<Vec<Square>> = Vec::with_capacity(height);
        for r in 0..height {
            let mut row = Vec::new();
            
            for c in 0..width {
                row.push(Square::with_brush(r, c, brush.clone()));
            }
            
            quilt.push(row);
        }
        
        let mut images = Vec::with_capacity(width * height);
        for index in 0..width*height {


            let mut image = Image::new(Square::SQUARE_WIDTH as i32, Square::SQUARE_WIDTH as i32);

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
            let (tx, rx) = mpsc::channel::<(Square, f64, usize, Arc<Mutex<(bool, bool)>>)>();

            Quilt::start_render_thread(rx, ready_tx.clone());

            thread_streams.push(tx);
        }

        let queued_squares = vec![Arc::new(Mutex::new((false, false))); width * height].into();

        Quilt {
            width: width,
            height: height,
            quilt: quilt,
            images: images,
            undo_redo: UndoRedo::new(),

            ready_tx: ready_tx,
            thread_streams: thread_streams,
            next_thread_index: 0,
            queued_squares
        }
    }

    pub fn queue_draw(&mut self, index: usize, scale: f64) {
        if let Some(status) = self.queued_squares.get(index) {
            (*status.lock().unwrap()).0 = false;
        }
        self.next_thread_index = self.next_thread_index % Quilt::NUM_THREADS as u8;

        let transmitter = &self.thread_streams[self.next_thread_index as usize];
        let row = index as usize / self.width;
        let column = index as usize % self.width;
        let square = &self.quilt[row][column];
        let status = Arc::new(Mutex::new((true, false)));

        self.queued_squares[index] = status.clone();
        let _ = transmitter.send((square.clone(), scale + Quilt::OVER_SCALE, index, status.clone()));

        self.next_thread_index += 1;
    }

    pub fn queue_complete_redraw(&mut self, scale: f64) {
        let num_squares = self.width * self.height;

        for i in 0..num_squares {
            self.queue_draw(i, scale);
        }
    }

    fn is_square_on_screen(row: f64, col: f64, scale: f64, rect: &Rectangle<f64>) -> bool {
        let top_left_x = Square::SQUARE_WIDTH * col as f64 * scale + rect.x;
        let top_left_y = Square::SQUARE_WIDTH * row as f64 * scale + rect.y;

        (top_left_x < rect.width  && top_left_x > -Square::SQUARE_WIDTH * scale) &&
        (top_left_y < rect.height && top_left_y > -Square::SQUARE_WIDTH * scale)
    }
    
    pub fn queue_window_redraw(&mut self, scale: f64, rect: &Rectangle<f64>) {
        for row in 0..self.height {
            for column in 0..self.width {

                if Quilt::is_square_on_screen(row as f64, column as f64, scale, rect) {
                    
                    self.queue_draw(row * self.width + column, scale);

                } else {
                    if let Some(status) = self.queued_squares.get(row * self.width + column) {
                        (*status.lock().unwrap()).0 = false;
                    }
                }
            }
        }
    }

    // spawn a thread that will update the image buffer
    fn start_render_thread(rx: mpsc::Receiver<(Square, f64, usize, Arc<Mutex<(bool, bool)>>)>, ready_tx: glib::Sender<((Image, f64), usize)>) {
        thread::spawn(move || {
            for (square, scale, index, status) in rx.iter() {
                if let Ok(lock) = status.try_lock() {
                    if !(lock.0 || lock.1) {
                        continue
                    }
                }

                let mut image = Image::new((Square::SQUARE_WIDTH * scale) as i32, (Square::SQUARE_WIDTH * scale) as i32);
                image.with_surface(|surface| {
                    let cr = cairo::Context::new(&surface);

                    cr.scale(scale, scale);

                    square.draw(&cr);
                });
                
                let _ = ready_tx.send(((image, scale), index));
            }
        });
    }

    pub fn draw(&mut self, cr: &Context, camera_transform: Arc<Mutex<CameraTransform>>, rect: &Rectangle<f64>) {
        cr.save();

        cr.move_to(0.0, 0.0);
        cr.line_to(Square::SQUARE_WIDTH * self.width as f64, 0.0);
        cr.line_to(Square::SQUARE_WIDTH * self.width as f64, Square::SQUARE_WIDTH * self.height as f64);
        cr.line_to(0.0, Square::SQUARE_WIDTH * self.height as f64);
        cr.line_to(0.0, 0.0);

        cr.clip();

        let width = self.width;
        let scale = camera_transform.lock().unwrap().get_scale();
        let mut images = self.images.lock().unwrap();

        for index in 0..images.len() {
            let row = index as usize / width;
            let column = index as usize % width;

            //Don't render squares that aren't on screen
            if !Quilt::is_square_on_screen(row as f64, column as f64, scale, rect) {
                continue;
            }

            let (image, scale) = &mut images[index];

            let scale = *scale;
            cr.save();
            cr.scale(1.0/scale, 1.0/scale);

            image.borrow_mut().with_surface(|surface| {

                cr.set_source_surface(surface, Square::SQUARE_WIDTH * column as f64 * scale, Square::SQUARE_WIDTH * row as f64 * scale);

                cr.paint();

                cr.set_source_rgb(0.0, 0.0, 0.0);

            });
            cr.restore();
        }

        cr.restore()

    }

    pub fn get_dimensions_pixel(&self) -> (f64, f64) {
        return (
            Square::SQUARE_WIDTH * self.width as f64, 
            Square::SQUARE_WIDTH * self.height as f64
        )
    }

    pub fn get_quilt(&self) -> &Vec<Vec<Square>> {
        &self.quilt
    }

    pub fn set_square(&mut self, row: usize, column: usize, scale: f64, new_square: Square) {
        if let Some(row) = self.quilt.get_mut(row) {
            if let Some(square) = row.get_mut(column) {
                *square = new_square;
            }
        }

        self.queue_draw(row * self.width + column, scale)
    }

    pub fn get_square(&self, row: usize, column: usize) -> Option<&Square> {
        if let Some(row) = self.quilt.get(row) {
            if let Some(square) = row.get(column) {
                return Some(square)
            }
        }

        None
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

        if  x < 0.0 || x >= self.width  as f64 * Square::SQUARE_WIDTH ||
            y < 0.0 || y >= self.height as f64 * Square::SQUARE_WIDTH  {
            
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
                    
                    cr.translate(Square::SQUARE_WIDTH, 0.0);
                }

                cr.restore();

                cr.translate(0.0, Square::SQUARE_WIDTH);

            }

            cr.restore();
            
        }
        
        cr.restore();

        result
    }
}
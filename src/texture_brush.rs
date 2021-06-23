use crate::quilt::square::Square;
use crate::util::image::Image;

use gdk_pixbuf::Pixbuf;
use std::sync::{Mutex};

//
// Used to paint areas with either a color or texture
// Brush is immutable
//

pub struct Texture {
    pub image: Image,
    pub scale: f64,
}

impl Texture {

    const IMAGE_SIZE: i32 = 800; // the side length of the shortest side of an image

    fn load_image(name: &str) -> Result<Image, glib::Error> {

        let (_pixbuf_format, width, height) = match Pixbuf::get_file_info(name) {
            Some(data) => data,
            None => return Err(glib::error::Error::new(glib::FileError::Failed, "Could not get file info"))
        };

        let aspect_ratio = width / height;

        // sets the shortest side to Texture::IMAGE_SIZE, but keeps the aspect ratio
        let (import_width, import_height) = match width < height {
            true => {
                (Texture::IMAGE_SIZE, Texture::IMAGE_SIZE * aspect_ratio)
            },
            false => {
                (Texture::IMAGE_SIZE * aspect_ratio, Texture::IMAGE_SIZE)
            }
        };

        // imported with the desire import dimensions helps improve performance
        // you don't need to be rendering a 6000x6000 image at full res you can get away with a somewhat low res image
        let buf = match Pixbuf::from_file_at_size(name, import_width, import_height) {
            Ok(buf) => buf,
            Err(err) => {
                println!("{:?}", err);
                return Err(err)
            },
        };

        let mut image = Image::new(import_width, import_height);

        let bytes = gdk_pixbuf::Pixbuf::read_pixel_bytes(&buf).unwrap();

        let b: &[u8] = &bytes;

        image.set_data(b);

        Ok(image)
    }

    pub fn new(file_path: &str) -> Result<Self, glib::Error> {
        let image: Image = match Texture::load_image(file_path){
            Ok(pixbuf) => pixbuf,
            Err(err) => return Err(err),
        };

        let small_side = std::cmp::min(image.get_width(), image.get_height());
        //scale required to fill the whole square of a quilt
        let scale = Square::SQUARE_WIDTH / small_side as f64;

        let s = Self {
            image: image,
            scale: scale,
        };

        Ok(s)
    }
}

#[allow(dead_code)]
pub struct TextureBrush {
    color: Option<(f64, f64, f64)>,
    texture: Option<Mutex<Texture>>,
}

impl TextureBrush {

    pub fn new() -> Self {
        Self {
            color: Some((1.0, 1.0, 1.0)),
            texture: None,
        }
    }

    pub fn new_color(color: (f64, f64, f64)) -> Self {
        Self {
            color: Some(color),
            texture: None,
        }
    }

    pub fn new_texture(texture: Texture) -> Self {
        Self {
            color: None,
            texture: Some(Mutex::new(texture)),
        }
    }

    pub fn try_new_texture(path: &str) -> Result<Self, glib::Error> {
        let texture = match Texture::new(path) {
            Ok(texture) => {
                texture
            },
            Err(err) => {
                return Err(err);
            }
        };

        Ok(TextureBrush::new_texture(texture))
    }

    pub fn apply(&self, cr: &cairo::Context) {
        cr.save();

        if let Some(color) = self.color {
            cr.set_source_rgb(color.0, color.1, color.2);
            cr.fill();
        }

        if let Some(texture) = &self.texture {
            let mut texture = texture.lock().unwrap();
            let scale = texture.scale;

            texture.image.with_surface(|surface| {
                cr.save();
                cr.clip();
                cr.scale(scale, scale);
                cr.set_source_surface(surface, 0.0, 0.0);
                cr.paint();
                cr.restore();
            });
        }

        cr.restore();
    }

    pub fn get_color(&self) -> Option<(f64, f64, f64)> {
        self.color
    }
}
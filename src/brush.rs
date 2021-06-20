use gdk_pixbuf::Pixbuf;
use gdk::prelude::*;
use crate::quilt;

//
// Used to paint areas with either a color or texture
// Brush is immutable
//

pub struct Texture {
    pub pixbuf: Pixbuf,
    pub scale: f64,
}

impl Texture {

    const IMAGE_SIZE: i32 = 400; // the side length of the shortest side of an image

    fn load_image(name: &str) -> Result<Pixbuf, glib::Error> {

        // bad way to find the original size of the image
        let og_buf = match Pixbuf::from_file(name) {
            Ok(buf) => buf,
            Err(err) => return Err(err),
        };

        let aspect_ratio = og_buf.get_width() / og_buf.get_height();

        // sets the shortest side to Texture::IMAGE_SIZE, but keeps the aspect ratio
        let (import_width, import_height) = match og_buf.get_width() < og_buf.get_height() {
            true => {
                (Texture::IMAGE_SIZE, Texture::IMAGE_SIZE * aspect_ratio)
            },
            false => {
                (Texture::IMAGE_SIZE * aspect_ratio, Texture::IMAGE_SIZE)
            }
        };

        // imported with the desire import dimensions helps improve performance
        // you don't need to be rendering a 6000x6000 image at full res you can get away with a somewhat low res image
        match Pixbuf::from_file_at_size(name, import_width, import_height) {
            Ok(buf) => Ok(buf),
            Err(err) => {
                println!("{:?}", err);
                Err(err)
            },
        }
    }

    pub fn new(file_path: &str) -> Result<Self, glib::Error> {
        let image: Pixbuf = match Texture::load_image(file_path){
            Ok(pixbuf) => pixbuf,
            Err(err) => return Err(err),
        };

        let small_side = std::cmp::min(image.get_width(), image.get_height());
        //scale required to fill the whole square of a quilt
        let scale = quilt::SQUARE_WIDTH / small_side as f64;

        let s = Self {
            pixbuf: image,
            scale: scale,
        };

        Ok(s)
    }

}

#[allow(dead_code)]
pub struct Brush {
    color: Option<(f64, f64, f64)>,
    texture: Option<Texture>,
}

impl Brush {

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
            texture: Some(texture),
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

        Ok(Brush::new_texture(texture))
    }

    pub fn apply(&self, cr: &cairo::Context) {
        cr.save();

        if let Some(color) = self.color {
            cr.set_source_rgb(color.0, color.1, color.2);
            cr.fill();
        }

        if let Some(texture) = &self.texture {
            cr.clip();
            cr.scale(texture.scale, texture.scale);
            cr.set_source_pixbuf(&texture.pixbuf, 0.0, 0.0);
            cr.paint();
        }

        cr.restore();
    }

    pub fn get_color(&self) -> Option<(f64, f64, f64)> {
        self.color
    }
}
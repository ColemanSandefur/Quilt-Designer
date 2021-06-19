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

    fn load_image(name: &str) -> Result<Pixbuf, glib::Error> {
        match Pixbuf::from_file(name) {
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

}
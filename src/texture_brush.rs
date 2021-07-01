use crate::quilt::square::Square;
use crate::util::image::Image;
use crate::parser::{Parser, Serializer, Savable, SerializeData, ParseData, SaveData};

use yaml_rust::Yaml;
use gdk_pixbuf::Pixbuf;
use std::sync::{Mutex};
use gdk::prelude::*;

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

        let (_pixbuf_format, width, height) = match Pixbuf::file_info(name) {
            Some(data) => data,
            None => return Err(glib::error::Error::new(glib::FileError::Failed, "Could not get file info"))
        };

        let aspect_ratio = width as f64 / height as f64;

        // sets the shortest side to Texture::IMAGE_SIZE, but keeps the aspect ratio
        let (import_width, import_height) = match width < height {
            true => {
                (Texture::IMAGE_SIZE, (Texture::IMAGE_SIZE as f64 * aspect_ratio) as i32)
            },
            false => {
                ((Texture::IMAGE_SIZE as f64 * aspect_ratio) as i32, Texture::IMAGE_SIZE)
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

        // write the pixbuf to a thread-safe image structure
        let mut image = Image::new(import_width, import_height);

        image.with_surface(|surface| {
            let cr = cairo::Context::new(&surface).unwrap();

            cr.set_source_pixbuf(&buf, 0.0, 0.0);
            cr.paint().unwrap();
            cr.set_source_rgb(0.0, 0.0, 0.0);
        });

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

// will pass "./saves/{save_name}" to image
impl Savable for Texture {
    fn to_save(&self, save_path: &mut SaveData) -> Yaml {
        let yaml = Serializer::create_map(vec![
            ("scale", Serializer::serialize(self.scale)),
            ("image", self.image.to_save(save_path))
        ]);

        yaml
    }

    fn from_save(yaml: &Yaml, save_path: &mut SaveData) -> Box<Self> {
        let map = Parser::to_map(yaml);

        let scale = Parser::parse(map.get(&Serializer::from_str("scale")).unwrap());
        let image = Image::from_save(map.get(&Serializer::from_str("image")).unwrap(), save_path);

        Box::new(Self {
            scale,
            image: *image
        })
    }
}

impl std::fmt::Display for Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "width: {}, height: {}, scale: {}", self.image.get_width(), self.image.get_height(), self.scale)
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
        cr.save().unwrap();

        if let Some(color) = self.color {
            cr.set_source_rgb(color.0, color.1, color.2);
            cr.fill().unwrap();
        }

        if let Some(texture) = &self.texture {
            let mut texture = texture.lock().unwrap();
            let scale = texture.scale;

            texture.image.with_surface(|surface| {
                cr.save().unwrap();
                cr.clip();
                cr.scale(scale, scale);
                cr.set_source_surface(surface, 0.0, 0.0).unwrap();
                cr.paint().unwrap();
                cr.restore().unwrap();
            });
        }

        cr.restore().unwrap();
    }

    pub fn get_color(&self) -> Option<(f64, f64, f64)> {
        self.color
    }
}

impl std::fmt::Display for TextureBrush {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // match self.color {
        //     Some(color) => write!(f, "color: (R: {}, G: {}, B: {})", color.0, color.1, color.2)
        //     None => 
        // }

        if let Some(color) = self.color {
            return write!(f, "color: (R: {}, G: {}, B: {})", color.0, color.1, color.2)
        }

        if let Some(texture) = &self.texture {
            return write!(f, "texture: {}", texture.lock().unwrap())
        }

        write!(f, "No color or texture")
    }
}

impl Savable for TextureBrush {
    fn to_save(&self, save_path: &mut SaveData) -> Yaml {
        if let Some(color) = self.color {
            return Serializer::create_map(vec!{
                ("color", Serializer::create_map(vec!{
                    ("r", Serializer::serialize(color.0)),
                    ("g", Serializer::serialize(color.1)),
                    ("b", Serializer::serialize(color.2))
                }))
            });
        } else {
            let texture = self.texture.as_ref().unwrap().lock().unwrap();

            return Serializer::create_map(vec!{
                ("texture", texture.to_save(save_path))
            });
        }
    }

    fn from_save(yaml: &Yaml, save_path: &mut SaveData) -> Box<Self> {
        let map = Parser::to_map(yaml);

        if let Some(color_yaml) = map.get(&Serializer::from_str("color")) {
            let colors = Parser::to_map(color_yaml);
            let color: (f64, f64, f64) = (
                Parser::parse(colors.get(&Serializer::serialize("r")).unwrap()),
                Parser::parse(colors.get(&Serializer::serialize("g")).unwrap()),
                Parser::parse(colors.get(&Serializer::serialize("b")).unwrap()),
            );

            return Box::new(Self {
                color: Some(color),
                texture: None,
            })
        } else {
            let texture = Texture::from_save(map.get(&Serializer::from_str("texture")).unwrap(), save_path);

            return Box::new(Self {
                color: None,
                texture: Some(Mutex::new(*texture)),
            })
        }
    }
}
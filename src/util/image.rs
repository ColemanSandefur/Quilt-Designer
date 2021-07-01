use cairo::{Format, ImageSurface};
use std::cell::RefCell;
use std::rc::Rc;
use crate::parser::{Parser, Serializer, Savable, SaveData};
use yaml_rust::Yaml;
use sha2::Digest;
use crate::gtk::prelude::*;
use std::io::{Read, Write};

#[derive(Clone)]
pub struct Image {
    width: i32,
    height: i32,
    data: Option<Box<[u8]>>,
    stride: i32,
}

impl Image {
    pub const FORMAT_TYPE: Format = Format::Rgb24;

    pub fn new (width: i32, height: i32) -> Self {
        let stride = Format::stride_for_width(Image::FORMAT_TYPE, width as u32).unwrap();

        Self {
            width,
            height,
            data: Some(vec![0; stride as usize * height as usize].into()), // rgba each has a byte allocated to it
            stride: stride
        }
    }

    pub fn new_with_data(width: i32, height: i32, data: Box<[u8]>) -> Self {
        let stride = Format::stride_for_width(Image::FORMAT_TYPE, width as u32).unwrap();

        Self {
            width,
            height,
            data: Some(data),
            stride: stride
        }
    }

    pub fn with_surface<F: FnOnce(&ImageSurface)>(&mut self, func: F) {
        let image = self.data.take().expect("Empty image");

        let return_location = Rc::new(RefCell::new(None));
        {
            let holder = ImageHolder::new(Some(image), return_location.clone());

            let surface = ImageSurface::create_for_data(
                holder,
                Image::FORMAT_TYPE,
                self.width,
                self.height,
                self.stride, // how many bytes until next row (r, g, b, a, all have a byte allocated thats why it is 4*width)
            )
            .expect("Can't create surface");
            func(&surface);
        }

        self.data = Some(
            return_location
                .borrow_mut()
                .take()
                .expect("Image not returned"),
        );
    }

    pub fn to_surface(mut self) -> std::result::Result<cairo::ImageSurface, cairo::Error> {
        let image = self.data.take().expect("Empty image");

        let surface = ImageSurface::create_for_data(
            image,
            Image::FORMAT_TYPE,
            self.width,
            self.height,
            self.stride, // how many bytes until next row (r, g, b, a, all have a byte allocated thats why it is 4*width)
        );

        surface
    }

    pub fn to_pixbuf(self) -> gdk_pixbuf::Pixbuf {
        let width = self.width;
        let height = self.height;

        let mut pixbuf_surface = cairo::ImageSurface::create(cairo::Format::ARgb32, self.width, self.height).unwrap();
        let cr = cairo::Context::new(&pixbuf_surface).unwrap();

        cr.set_source_surface(&self.to_surface().unwrap(), 0.0, 0.0).unwrap();
        cr.paint().unwrap();
        cr.set_source_rgb(0.0, 0.0, 0.0);

        let stride = pixbuf_surface.stride();
        let mut data = pixbuf_surface.data().unwrap();

        // convert bytes from BGRA to RGBA
        for index in (3..data.len()).step_by(4) {
            let alpha = data[index - 0];
            let blue = data[index - 3];
            let red = data[index - 1];
            let green = data[index - 2];

            data[index - 0] = alpha; // alpha
            data[index - 1] = blue;  // blue
            data[index - 2] = green; // green
            data[index - 3] = red;   // red
        }

        let pixbuf = gdk_pixbuf::Pixbuf::from_bytes(
            &glib::Bytes::from(&*data),
            gdk_pixbuf::Colorspace::Rgb,
            true,
            8,
            width,
            height,
            stride
        );

        pixbuf
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }
}

impl Savable for Image {
    fn to_save(&self, save_path: &mut SaveData) -> Yaml {
        
        let pixbuf = gdk_pixbuf::Pixbuf::from(self.clone());
        
        // hash pixbuf's contents to create a unique name
        let mut hasher = sha2::Sha256::new();
        
        let buffer = pixbuf.save_to_bufferv("png", &[]).unwrap();
        
        hasher.update(&buffer);
        
        let result: Vec<u8> = hasher.finalize().to_vec();
        
        // convert result to a string
        let file_name: String = format!("{}.png", result.into_iter().map(|i| i.to_string()).collect::<String>());

        // bad way of preventing a file from being written multiple times
        if !save_path.files_written.contains(&file_name) {
            save_path.files_written.push(file_name.clone());
            let mut writer = save_path.writer.as_ref().unwrap().lock().unwrap();
            
            let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
            writer.start_file(file_name.clone(), options).unwrap();
    
            writer.write(&buffer).unwrap();
        }

        let serialized = Serializer::create_map(vec![
            ("width", Serializer::from_i64(self.width as i64)),
            ("height", Serializer::from_i64(self.height as i64)),
            ("data_location", Serializer::from_str(file_name.as_str())),
        ]);

        serialized
    }

    fn from_save(yaml: &Yaml, save_path: &mut SaveData) -> Box<Self> {
        
        let map = Parser::to_map(yaml);
        
        let width = Parser::to_i64(map.get(&Serializer::from_str("width")).unwrap()) as i32;
        let height = Parser::to_i64(map.get(&Serializer::from_str("height")).unwrap()) as i32;
        let file_name = String::from(Parser::to_str(map.get(&Serializer::from_str("data_location")).unwrap()));
        
        let mut vec = Vec::new();
        {
            let mut reader = save_path.reader.as_ref().unwrap().lock().unwrap();
            let mut file = reader.by_name(&file_name).unwrap();
            file.read_to_end(&mut vec).unwrap();
        }

        let buf = gdk_pixbuf::Pixbuf::from_read(std::io::Cursor::new(vec)).unwrap();

        // write the pixbuf to a thread-safe image structure
        let mut image = Image::new(width, height);

        image.with_surface(|surface| {
            let cr = cairo::Context::new(&surface).unwrap();

            cr.set_source_pixbuf(&buf, 0.0, 0.0);
            cr.paint().unwrap();
            cr.set_source_rgb(0.0, 0.0, 0.0);
        });

        Box::new(image)
    }
}

impl From<Image> for gdk_pixbuf::Pixbuf {
    fn from(image: Image) -> gdk_pixbuf::Pixbuf {
        let width = image.width;
        let height = image.height;

        let mut pixbuf_surface = cairo::ImageSurface::create(cairo::Format::ARgb32, image.width, image.height).unwrap();
        {
            let cr = cairo::Context::new(&pixbuf_surface).unwrap();
    
            cr.set_source_surface(&image.to_surface().unwrap(), 0.0, 0.0).unwrap();
            cr.paint().unwrap();
            cr.set_source_rgb(0.0, 0.0, 0.0);
        }

        let stride = pixbuf_surface.stride();
        let mut data = pixbuf_surface.data().unwrap();

        // convert bytes from BGRA to RGBA
        for index in (3..data.len()).step_by(4) {
            let alpha = data[index - 0];
            let blue = data[index - 3];
            let red = data[index - 1];
            let green = data[index - 2];

            data[index - 0] = alpha; // alpha
            data[index - 1] = blue;  // blue
            data[index - 2] = green; // green
            data[index - 3] = red;   // red
        }

        let pixbuf = gdk_pixbuf::Pixbuf::from_bytes(
            &glib::Bytes::from(&*data),
            gdk_pixbuf::Colorspace::Rgb,
            true,
            8,
            width,
            height,
            stride
        );

        pixbuf
    }
}

pub struct ImageHolder {
    image: Option<Box<[u8]>>,
    return_location: Rc<RefCell<Option<Box<[u8]>>>>,
}

impl ImageHolder {
    pub fn new(image: Option<Box<[u8]>>, return_location: Rc<RefCell<Option<Box<[u8]>>>>) -> Self {
        Self {
            image,
            return_location,
        }
    }
}

/// This stores the pixels back into the return_location as now nothing
/// references the pixels anymore
impl Drop for ImageHolder {
    fn drop(&mut self) {
        *self.return_location.borrow_mut() = Some(self.image.take().expect("Holding no image"));
    }
}

/// Needed for ImageSurface::create_for_data() to be able to access the pixels
impl AsRef<[u8]> for ImageHolder {
    fn as_ref(&self) -> &[u8] {
        self.image.as_ref().expect("Holding no image").as_ref()
    }
}

impl AsMut<[u8]> for ImageHolder {
    fn as_mut(&mut self) -> &mut [u8] {
        self.image.as_mut().expect("Holding no image").as_mut()
    }
}
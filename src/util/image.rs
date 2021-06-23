use cairo::{Format, ImageSurface};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Image {
    width: i32,
    height: i32,
    data: Option<Box<[u8]>>,
    stride: i32,
}

impl Image {
    const FORMAT_TYPE: Format = Format::ARgb32;

    pub fn new (width: i32, height: i32) -> Self {
        let stride = Format::stride_for_width(Image::FORMAT_TYPE, width as u32).unwrap();

        Self {
            width,
            height,
            data: Some(vec![0; stride as usize * height as usize].into()), // rgba each has a byte allocated to it
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

    pub fn set_data(&mut self, data: &[u8]) {
        let mut image_data = self.data.take().expect("Empty image");

        let min_len = std::cmp::min(image_data.len(), data.len());

        if data.len() as f64 / image_data.len() as f64 == 0.75 {
            let mut num_inserted = 0;

            for index in (2..data.len()).step_by(3) {
                num_inserted += 1;

                image_data[index + num_inserted - 3] = data[index - 0]; // red
                image_data[index + num_inserted - 2] = data[index - 1]; // green
                image_data[index + num_inserted - 1] = data[index - 2]; // blue
                image_data[index + num_inserted - 0] = 255;             // alpha
            }
        } else {
            let mut num_inserted = 0;

            for index in (3..min_len).step_by(4) {
                num_inserted += 1;

                image_data[index + num_inserted - 3] = data[index - 0]; // red
                image_data[index + num_inserted - 2] = data[index - 1]; // green
                image_data[index + num_inserted - 1] = data[index - 2]; // blue
                image_data[index + num_inserted - 0] = data[index - 3]; // alpha
            }
        }


        self.data = Some(image_data);
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
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
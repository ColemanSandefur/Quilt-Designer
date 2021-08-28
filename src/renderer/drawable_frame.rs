use std::cell::RefCell;
use std::rc::Rc;
use glium::framebuffer::SimpleFrameBuffer;

pub struct DrawableFrame {
    // picking_pixel_buffer: glium::texture::pixel_buffer::PixelBuffer<u32>,
    // picking_attachments: Option<(glium::texture::UnsignedTexture2d, glium::framebuffer::DepthRenderBuffer)>,

    target_color: RefCell<Option<glium::texture::Texture2d>>,
    target_depth: RefCell<Option<glium::framebuffer::DepthRenderBuffer>>,

    display: Rc<glium::Display>,
}


impl DrawableFrame {
    pub fn new(display: Rc<glium::Display>) -> Self {
        Self {
            target_color: RefCell::new(None),
            target_depth: RefCell::new(None),
            display,
        }
    }

    pub fn draw(&self, surface: &mut impl glium::Surface, draw: impl FnOnce(&mut SimpleFrameBuffer<'_>)) {
        let surface_dimensions = surface.get_dimensions();

        let mut target_color = self.target_color.borrow_mut();
        let mut target_depth = self.target_depth.borrow_mut();

        {  
            let clear = if let Some(ref tex) = target_color.as_ref() {
                tex.get_width() != surface_dimensions.0 || tex.get_height().unwrap() != surface_dimensions.1 
            } else {false};

            if clear {
                *target_color = None;
            }
        }

        {
            let clear = if let Some(ref tex) = target_depth.as_ref() {
                tex.get_dimensions() != surface_dimensions
            } else {false};

            if clear {
                *target_depth = None;
            }
        }

        if target_color.is_none() {
            let texture = glium::texture::Texture2d::empty(&*self.display, surface_dimensions.0 as u32, surface_dimensions.1 as u32).unwrap();

            *target_color = Some(texture);
        }
        let target_color = target_color.as_ref().unwrap();

        if target_depth.is_none() {
            let texture = glium::framebuffer::DepthRenderBuffer::new(&*self.display, glium::texture::DepthFormat::I24, surface_dimensions.0 as u32, surface_dimensions.1 as u32).unwrap();

            *target_depth = Some(texture);
        }
        let target_depth = target_depth.as_ref().unwrap();

        draw(&mut SimpleFrameBuffer::with_depth_buffer(&*self.display, target_color,
            target_depth).unwrap());
    }

    pub fn get_texture(&self) -> &RefCell<Option<glium::texture::Texture2d>> {
        &self.target_color
    }

    pub fn fxaa(&self, surface: &mut impl glium::Surface, draw: impl FnOnce(&mut SimpleFrameBuffer<'_>)) {
        self.draw(surface, draw);

        
    }
}
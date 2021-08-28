use std::cell::RefCell;
use std::rc::Rc;
use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::DepthTexture2dMultisample;

pub struct DrawableFrame {
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
}

pub struct MultisampleDrawableFrame {
    target_color: RefCell<Option<glium::texture::Texture2dMultisample>>,
    target_depth: RefCell<Option<DepthTexture2dMultisample>>,

    display: Rc<glium::Display>,
}


impl MultisampleDrawableFrame {
    pub fn new(display: Rc<glium::Display>) -> Self {
        Self {
            target_color: RefCell::new(None),
            target_depth: RefCell::new(None),
            display,
        }
    }

    pub fn draw(&self, surface: &mut impl glium::Surface, samples: u32, draw: impl FnOnce(&mut SimpleFrameBuffer<'_>)) {
        let surface_dimensions = surface.get_dimensions();

        let mut target_color = self.target_color.borrow_mut();
        let mut target_depth = self.target_depth.borrow_mut();

        {  
            let clear = if let Some(ref tex) = target_color.as_ref() {
                tex.get_width() != surface_dimensions.0 || tex.get_height().unwrap() != surface_dimensions.1 || tex.samples() != samples
            } else {false};

            if clear {
                *target_color = None;
            }
        }

        {
            let clear = if let Some(ref tex) = target_depth.as_ref() {
                tex.get_width() != surface_dimensions.0 || tex.get_height().unwrap() != surface_dimensions.1
            } else {false};

            if clear {
                *target_depth = None;
            }
        }

        if target_color.is_none() {
            let texture = glium::texture::Texture2dMultisample::empty(&*self.display, surface_dimensions.0 as u32, surface_dimensions.1 as u32, samples).unwrap();

            *target_color = Some(texture);
        }
        let target_color = target_color.as_ref().unwrap();

        if target_depth.is_none() {
            let texture = DepthTexture2dMultisample::empty(&*self.display, surface_dimensions.0 as u32, surface_dimensions.1 as u32, samples).unwrap();

            *target_depth = Some(texture);
        }
        let target_depth = target_depth.as_ref().unwrap();

        draw(&mut SimpleFrameBuffer::with_depth_buffer(&*self.display, target_color,
            target_depth).unwrap());
    }

    pub fn get_texture(&self) -> &RefCell<Option<glium::texture::Texture2dMultisample>> {
        &self.target_color
    }
}
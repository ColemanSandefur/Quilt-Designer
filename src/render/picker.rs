use crate::render::renderer::Renderer;

pub struct Picker {
    pub picking_pixel_buffer: glium::texture::pixel_buffer::PixelBuffer<u32>,
    pub picking_attachments: Option<(glium::texture::UnsignedTexture2d, glium::framebuffer::DepthRenderBuffer)>,
}

impl Picker {
    pub fn new(display: &dyn glium::backend::Facade) -> Self {

        Self {
            picking_pixel_buffer: glium::texture::pixel_buffer::PixelBuffer::new_empty(display, 1),
            picking_attachments: None,
        }
    }

    pub fn get_clicked(&mut self) {
        self.picking_pixel_buffer.read().map(|d| d[0]).unwrap_or(0);
    }

    pub fn draw(&mut self, _renderer: &mut Renderer, _target: &mut glium::Frame, _global_transform: &crate::render::matrix::WorldTransform) {
        // renderer.quilt.draw_click(target, global_transform, &Default::default());
    }
}
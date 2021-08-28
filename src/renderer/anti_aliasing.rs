use crate::renderer::vertex::Vertex;
use crate::renderer::matrix::WorldTransform;
use crate::renderer::drawable_frame::MultisampleDrawableFrame;

use std::cell::RefCell;
use glium::Surface;
use std::rc::Rc;

pub enum AntiAliasMode{
    NONE,
    FXAA,
    MSAA(u32),
}

pub struct AntiAliasing {
    mode: AntiAliasMode,

    fxaa: Fxaa,
    msaa: Msaa,
}

impl AntiAliasing {
    pub fn new(display: Rc<glium::Display>) -> Self {
        Self {
            mode: AntiAliasMode::MSAA(16),
            fxaa: Fxaa::new(display.clone()),
            msaa: Msaa::new(display.clone()),
        }
    }

    pub fn draw(&mut self, surface: &mut impl glium::Surface, shape: &(&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>), global_transform: &WorldTransform) {
        match &self.mode {
            AntiAliasMode::NONE => {
                surface.clear_color(0.02, 0.02, 0.02, 1.0);

                crate::renderer::material::get_material_manager().get_solid_color_material().draw(shape, surface, global_transform, &Default::default());
            },

            AntiAliasMode::MSAA(samples) => {
                self.msaa.draw(surface, *samples, |frame| {
                    frame.clear_color(0.02, 0.02, 0.02, 1.0);

                    crate::renderer::material::get_material_manager().get_solid_color_material().draw(shape, frame, global_transform, &Default::default());
                });
            }

            AntiAliasMode::FXAA => {
                self.fxaa.draw(surface, |frame| {
                    frame.clear_color(0.02, 0.02, 0.02, 1.0);

                    crate::renderer::material::get_material_manager().get_solid_color_material().draw(shape, frame, global_transform, &Default::default());
                }, true);
            }
        }
    }

    pub fn set_multi_sample_mode(&mut self, sampling_mode: AntiAliasMode) {
        self.mode = sampling_mode;
    }

    pub fn get_multi_sample_mode(&self) -> &AntiAliasMode {
        &self.mode
    }
}

pub use fxaa::*;
mod fxaa {
    use crate::renderer::drawable_frame::DrawableFrame;
use crate::renderer::vertex::Vertex;

use std::rc::{Rc};
use glium::{VertexBuffer, IndexBuffer};
use glium::framebuffer::SimpleFrameBuffer;

#[allow(dead_code)]
pub struct Fxaa {
    display: Rc<glium::Display>,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,

    frame: DrawableFrame,
}

impl Fxaa {
    pub fn new(display: Rc<glium::Display>) -> Self {
        let vertex_buffer = glium::VertexBuffer::new(&*display,
            &[
                Vertex { position: [-1.0, -1.0], .. Default::default() },
                Vertex { position: [-1.0,  1.0], .. Default::default() },
                Vertex { position: [ 1.0,  1.0], .. Default::default() },
                Vertex { position: [ 1.0, -1.0], .. Default::default() },
            ]
        ).unwrap();

        let index_buffer = glium::index::IndexBuffer::new(&*display,
                    glium::index::PrimitiveType::TriangleStrip, &[1 as u32, 2, 0, 3]).unwrap();

        let frame = DrawableFrame::new(display.clone());

        Self {
            vertex_buffer,
            index_buffer,

            frame,

            display,
        }
    }

    pub fn draw(&self, surface: &mut impl glium::Surface, draw: impl FnOnce(&mut SimpleFrameBuffer<'_>), fxaa_enabled: bool) {
        self.frame.draw(surface, draw);

        crate::renderer::material::get_material_manager().get_fxaa_material().draw(&(&self.vertex_buffer, &self.index_buffer), surface, self.frame.get_texture().borrow().as_ref().unwrap(), fxaa_enabled, &Default::default());
    }
}
}

pub use msaa::*;
mod msaa {
    use super::*;
    use glium::framebuffer::SimpleFrameBuffer;
    use std::rc::Rc;
    use glium::{VertexBuffer, IndexBuffer};

    pub struct Msaa {
        surface: MultisampleDrawableFrame,
        vertex_buffer: VertexBuffer<Vertex>,
        index_buffer: IndexBuffer<u32>,
    }

    impl Msaa {
        pub fn new(display: Rc<glium::Display>) -> Self {
            let vertex_buffer = glium::VertexBuffer::new(&*display,
                &[
                    Vertex { position: [-1.0, -1.0], .. Default::default() },
                    Vertex { position: [-1.0,  1.0], .. Default::default() },
                    Vertex { position: [ 1.0,  1.0], .. Default::default() },
                    Vertex { position: [ 1.0, -1.0], .. Default::default() },
                ]
            ).unwrap();
    
            let index_buffer = glium::index::IndexBuffer::new(&*display,
                        glium::index::PrimitiveType::TriangleStrip, &[1 as u32, 2, 0, 3]).unwrap();

            Self {
                surface: MultisampleDrawableFrame::new(display.clone()),
                vertex_buffer,
                index_buffer,
            }
        }

        pub fn draw(&self, surface: &mut impl glium::Surface, samples: u32, draw: impl FnOnce(&mut SimpleFrameBuffer<'_>)) {
            self.surface.draw(surface, samples, draw);

            crate::renderer::material::get_material_manager().get_msaa_material().draw(&(&self.vertex_buffer, &self.index_buffer), surface, self.surface.get_texture().borrow().as_ref().unwrap(), samples as i32, &Default::default());
        }

        pub fn get_texture(&self) -> &RefCell<Option<glium::texture::Texture2dMultisample>> {
            self.surface.get_texture()
        }
    }
}

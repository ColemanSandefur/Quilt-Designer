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

        // let shape = PathShape::square(-1.0, -1.0, 2.0, 2.0, 0);

        // let verticies = vec! [
        //     Vertex { position: [-1.0, -1.0], .. Default::default() },
        //     Vertex { position: [-1.0,  1.0], .. Default::default() },
        //     Vertex { position: [ 1.0,  1.0], .. Default::default() },
        //     Vertex { position: [ 1.0, -1.0], .. Default::default() },
        // ];

        // let vertex_buffer = VertexBuffer::new(&*display, &shape.get_vertices()).unwrap();
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
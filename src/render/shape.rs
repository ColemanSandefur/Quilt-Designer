use glium::Surface;
use lyon::math::{point, Point};
use lyon::path::Path;
use lyon::tessellation::*;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2]
}

impl Vertex {
    pub fn to_point(&self) -> Point {
        point(self.position[0], self.position[1])
    }
}

implement_vertex!(Vertex, position);


pub trait Shape {
    fn get_vertex_buffer(&self) -> &glium::VertexBuffer<Vertex>;
    fn get_index_buffer(&self) -> &glium::IndexBuffer<u32>;
}

pub struct Square {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u32>,
}


impl Square {
    pub fn new(facade: &dyn glium::backend::Facade, pos1: (f32, f32), pos2: (f32, f32), pos3: (f32, f32), pos4: (f32, f32)) -> Self {
        let vertex_buffer = glium::VertexBuffer::new(facade, &[
            Vertex { position: [pos1.0, pos1.1] },
            Vertex { position: [pos2.0, pos2.1] },
            Vertex { position: [pos3.0, pos3.1] },
            Vertex { position: [pos4.0, pos4.1] },
        ]).unwrap();
        let index_buffer = glium::IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, &[0u32, 1, 2, 1, 2, 3]).unwrap();

        Self {
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn with_width_height(facade: &dyn glium::backend::Facade, x: f32, y: f32, width: f32, height: f32) -> Self {
        let vertex_buffer = glium::VertexBuffer::new(facade, &[
            Vertex { position: [ x, y ] },
            Vertex { position: [ x + width, y ] },
            Vertex { position: [ x, y + height ] },
            Vertex { position: [ x + width, y + height ] },
        ]).unwrap();
        let index_buffer = glium::IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, &[0u32, 1, 2, 1, 2, 3]).unwrap();

        Self {
            vertex_buffer,
            index_buffer,
        }
    }
}

impl Shape for Square {
    fn get_vertex_buffer(&self) -> &glium::VertexBuffer<Vertex> {
        &self.vertex_buffer
    }

    fn get_index_buffer(&self) -> &glium::IndexBuffer<u32> {
        &self.index_buffer
    }
}

pub struct PathShape {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u32>,
}

impl PathShape {
    pub fn new(facade: &dyn glium::backend::Facade, path: Path) -> Self {
        let mut geometry: VertexBuffers<Vertex, u32> = VertexBuffers::new();

        let mut tessellator = FillTessellator::new();

        {
            tessellator.tessellate(
                &path, 
                &FillOptions::default(), 
                &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
                    Vertex {
                        position: vertex.position().to_array(),
                    }
                }),
            ).unwrap();
        }

        let vertex_buffer = glium::VertexBuffer::new(facade, &geometry.vertices.to_vec()).unwrap();

        let index_buffer = glium::IndexBuffer::new(facade, glium::index::PrimitiveType::TriangleStrip, &geometry.indices.to_vec()).unwrap();

        Self {
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn from_vertices(facade: &dyn glium::backend::Facade, vertices: &Vec<Vertex>) -> Self {

        let mut builder = Path::builder();
        builder.begin(vertices[0].to_point());

        for i in 1..vertices.len() {
            builder.line_to(vertices[i].to_point());
        }
        builder.close();
        let path = builder.build();

        let mut geometry: VertexBuffers<Vertex, u32> = VertexBuffers::new();

        let mut tessellator = FillTessellator::new();

        {
            tessellator.tessellate_path(
                &path, 
                &FillOptions::default(), 
                &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
                    Vertex {
                        position: vertex.position().to_array(),
                    }
                }),
            ).unwrap();
        }

        let vertex_buffer = glium::VertexBuffer::new(facade, &geometry.vertices.to_vec()).unwrap();

        let index_buffer = glium::IndexBuffer::new(facade, glium::index::PrimitiveType::TriangleStrip, &geometry.indices.to_vec()).unwrap();

        Self {
            vertex_buffer,
            index_buffer,
        }
    }
}

impl Shape for PathShape {
    fn get_vertex_buffer(&self) -> &glium::VertexBuffer<Vertex> {
        &self.vertex_buffer
    }

    fn get_index_buffer(&self) -> &glium::IndexBuffer<u32> {
        &self.index_buffer
    }
}

pub fn draw<'a, U: glium::uniforms::Uniforms>(shape: &Box<dyn Shape>, frame: &mut glium::Frame, program: &glium::Program, uniforms: &U, draw_parameters: &glium::DrawParameters<'_>) {
    frame.draw(shape.get_vertex_buffer(), shape.get_index_buffer(), program, uniforms, draw_parameters).unwrap();
}
use crate::render::matrix::Matrix;

use glium::Surface;
use lyon::math::{point, Point};
use lyon::path::Path;
use lyon::tessellation::*;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub model: [[f32;4]; 4],
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: [0.0; 2],
            color: [1.0; 4],
            model: Matrix::new().get_matrix(),
        }
    }
}

impl Vertex {
    pub fn to_point(&self) -> Point {
        point(self.position[0], self.position[1])
    }
}

implement_vertex!(Vertex, position, color, model);


pub trait Shape {
    fn get_vertices(&mut self) -> Vec<Vertex>;
    fn get_indices(&mut self) -> Vec<u32>;
    fn set_color(&mut self, color: [f32; 4]);
    fn set_model_matrix(&mut self, matrix: Matrix);
}

pub struct Square {
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<u32>,
}

impl Square {
    pub fn new(pos1: (f32, f32), pos2: (f32, f32), pos3: (f32, f32), pos4: (f32, f32)) -> Self {
        let vertex_buffer = vec!{
            Vertex { position: [pos1.0, pos1.1], .. Default::default() },
            Vertex { position: [pos2.0, pos2.1], .. Default::default() },
            Vertex { position: [pos3.0, pos3.1], .. Default::default() },
            Vertex { position: [pos4.0, pos4.1], .. Default::default() },
        };

        let index_buffer = vec!{0u32, 1, 2, 1, 2, 3};

        Self {
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn with_width_height(x: f32, y: f32, width: f32, height: f32) -> Self {
        let vertex_buffer = vec!{
            Vertex { position: [ x, y ], .. Default::default() },
            Vertex { position: [ x + width, y ], .. Default::default() },
            Vertex { position: [ x, y + height ], .. Default::default() },
            Vertex { position: [ x + width, y + height ], .. Default::default() },
        };

        let index_buffer = vec!{0u32, 1, 2, 1, 2, 3};

        Self {
            vertex_buffer,
            index_buffer,
        }
    }
}

impl Shape for Square {
    fn get_vertices(&mut self) -> Vec<Vertex> {
        self.vertex_buffer.clone()
    }

    fn get_indices(&mut self) -> Vec<u32> {
        self.index_buffer.clone()
    }

    fn set_color(&mut self, color: [f32; 4]) {
        for vertex in &mut self.vertex_buffer {
            vertex.color = color;
        }
    }

    fn set_model_matrix(&mut self, matrix: Matrix) {
        for vertex in &mut self.vertex_buffer {
            vertex.model = matrix.get_matrix();
        }
    }
}

pub struct PathShape {
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<u32>,
}

impl PathShape {
    pub fn new(path: Path) -> Self {
        let mut geometry: VertexBuffers<Vertex, u32> = VertexBuffers::new();

        let mut tessellator = FillTessellator::new();

        {
            tessellator.tessellate(
                &path, 
                &FillOptions::default(), 
                &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
                    Vertex {
                        position: vertex.position().to_array(),
                        .. Default::default()
                    }
                }),
            ).unwrap();
        }

        let vertex_buffer = geometry.vertices.to_vec();

        let index_buffer = geometry.indices.to_vec();

        Self {
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn from_vertices(vertices: &Vec<Vertex>) -> Self {

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
                        .. Default::default()
                    }
                }),
            ).unwrap();
        }

        let vertex_buffer = geometry.vertices.to_vec();

        let index_buffer = geometry.indices.to_vec();

        Self {
            vertex_buffer,
            index_buffer,
        }
    }
}

impl Shape for PathShape {
    fn get_vertices(&mut self) -> Vec<Vertex> {
        self.vertex_buffer.clone()
    }

    fn get_indices(&mut self) -> Vec<u32> {
        self.index_buffer.clone()
    }

    fn set_color(&mut self, color: [f32; 4]) {
        for vertex in &mut self.vertex_buffer {
            vertex.color = color;
        }
    }

    fn set_model_matrix(&mut self, matrix: Matrix) {
        for vertex in &mut self.vertex_buffer {
            vertex.model = matrix.get_matrix();
        }
    }
}

pub fn draw<'a, U: glium::uniforms::Uniforms>(shape: &(&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>), frame: &mut glium::Frame, program: &glium::Program, uniforms: &U, draw_parameters: &glium::DrawParameters<'_>) {
    frame.draw(shape.0, shape.1, program, uniforms, draw_parameters).unwrap();
}
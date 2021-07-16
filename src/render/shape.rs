use crate::render::matrix::Matrix;

use lyon::math::{point, Point};
use lyon::path::Path;
use lyon::tessellation::*;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub model: [[f32;4]; 4],
    pub id: u32,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: [0.0; 2],
            color: [1.0; 4],
            model: Matrix::new().get_matrix(),
            id: 0,
        }
    }
}

impl Vertex {
    pub fn to_point(&self) -> Point {
        point(self.position[0], self.position[1])
    }
}

implement_vertex!(Vertex, position, color, model, id);


pub trait Shape {
    fn get_vertices(&mut self) -> Vec<Vertex>;
    fn get_indices(&mut self) -> Vec<u32>;
    fn set_color(&mut self, color: [f32; 4]);
    fn set_model_matrix(&mut self, matrix: Matrix);
    fn get_num_vertices(&mut self) -> usize;
    fn get_num_indices(&mut self) -> usize;
    fn get_id(&self) -> u32;
    fn set_id(&mut self, id: u32);
    fn was_clicked(&self, id: u32) -> bool {
        self.get_id() == id
    }
}

pub struct Square {
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<u32>,
}

impl Square {
    pub fn new(pos1: (f32, f32), pos2: (f32, f32), pos3: (f32, f32), pos4: (f32, f32), id: u32) -> Self {

        let vertex_buffer = vec!{
            Vertex { position: [pos1.0, pos1.1], id, .. Default::default() },
            Vertex { position: [pos2.0, pos2.1], id, .. Default::default() },
            Vertex { position: [pos3.0, pos3.1], id, .. Default::default() },
            Vertex { position: [pos4.0, pos4.1], id, .. Default::default() },
        };

        let index_buffer = vec!{0u32, 1, 2, 1, 2, 3};

        Self {
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn with_width_height(x: f32, y: f32, width: f32, height: f32, id: u32) -> Self {
        let vertex_buffer = vec!{
            Vertex { position: [ x, y ], id, .. Default::default() },
            Vertex { position: [ x + width, y ], id, .. Default::default() },
            Vertex { position: [ x, y + height ], id, .. Default::default() },
            Vertex { position: [ x + width, y + height ], id, .. Default::default() },
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

    fn get_num_vertices(&mut self) -> usize {
        self.vertex_buffer.len()
    }
    
    fn get_num_indices(&mut self) -> usize {
        self.index_buffer.len()
    }

    fn get_id(&self) -> u32 {
        match self.vertex_buffer.get(0) {
            Some(vertex) => vertex.id,
            None => 0,
        }
    }

    fn set_id(&mut self, id: u32) {
        for vertex in &mut self.vertex_buffer {
            vertex.id = id;
        }
    }
}

pub struct PathShape {
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<u32>,
}

impl PathShape {
    pub fn new(path: Path, id: u32) -> Self {
        let mut geometry: VertexBuffers<Vertex, u32> = VertexBuffers::new();

        let mut tessellator = FillTessellator::new();

        {
            tessellator.tessellate(
                &path, 
                &FillOptions::default(), 
                &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
                    Vertex {
                        position: vertex.position().to_array(),
                        id,
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

    pub fn from_vertices(vertices: &Vec<Vertex>, id: u32) -> Self {

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
                        id,
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

    fn get_num_vertices(&mut self) -> usize {
        self.vertex_buffer.len()
    }

    fn get_num_indices(&mut self) -> usize {
        self.index_buffer.len()
    }

    fn get_id(&self) -> u32 {
        match self.vertex_buffer.get(0) {
            Some(vertex) => vertex.id,
            None => 0,
        }
    }

    fn set_id(&mut self, id: u32) {
        for vertex in &mut self.vertex_buffer {
            vertex.id = id;
        }
    }
}

pub fn draw<'a, U: glium::uniforms::Uniforms>(shape: &(&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>), frame: &mut impl glium::Surface, program: &glium::Program, uniforms: &U, draw_parameters: &glium::DrawParameters<'_>) {
    frame.draw(shape.0, shape.1, program, uniforms, draw_parameters).unwrap();
}
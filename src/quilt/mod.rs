pub mod square;

use crate::render::material::{material_manager::{MaterialManager, MaterialType}};
use crate::render::matrix::{WorldTransform};
use crate::render::shape::Vertex;
use crate::render::material::{Material};
use crate::render::matrix::Matrix;
use square::Square;

use glium::{VertexBuffer, IndexBuffer};

#[derive(Default, Debug)]
pub struct DrawStats {
    pub draws: usize,
    pub vertices: usize,
    pub indices: usize,
}

impl DrawStats {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn reset(&mut self) {
        self.draws = 0;
        self.vertices = 0;
        self.indices = 0;
    }
}

#[allow(dead_code)]
pub struct Quilt {
    width: usize,
    height: usize,
    squares: Vec<Vec<Square>>,
    vert_vec: Vec<Vertex>,
    index_vec: Vec<u32>,
    vertex_buffers: VertexBuffer<Vertex>,
    index_buffers: IndexBuffer<u32>,
    pub shader: Box<dyn Material>,
    needs_updated: bool,
    pub draw_stats: DrawStats,
}

impl Quilt {

    pub const MAX_VERTICES: usize = 30000;
    pub const MAX_INDICES: usize = Self::MAX_VERTICES * 4;

    pub fn new(display: &dyn glium::backend::Facade, shaders: &mut MaterialManager, width: usize, height: usize) -> Self {

        let mut squares = Vec::with_capacity(height);

        for r in 0..height {
            let mut row = Vec::with_capacity(width);

            for c in 0..width {
                let mut square = Square::new(shaders);

                let column = c as f32;
                let r = -1.0 * r as f32 - 1.0;

                let mut transform = square.get_model_transform();
                transform.translate(column - width as f32 / 2.0, r + height as f32 / 2.0, 0.0);

                square.set_model_transform(transform);
                row.push(square);
            }

            squares.push(row);
        }

        println!("Finished loading squares");

        let vertex_buffers= VertexBuffer::empty_dynamic(display, Self::MAX_VERTICES).unwrap();
        let index_buffers = IndexBuffer::empty_dynamic(display, glium::index::PrimitiveType::TrianglesList, Self::MAX_INDICES).unwrap();

        println!("Loaded buffers");

        let vert_vec = Vec::with_capacity(Self::MAX_VERTICES);
        let index_vec = Vec::with_capacity(Self::MAX_INDICES);

        println!("Allocated vecs");

        Self {
            width,
            height,
            squares,
            vertex_buffers,
            index_buffers,
            shader: shaders.get_material(MaterialType::SolidColorMaterial).unwrap(),
            needs_updated: true,
            vert_vec,
            index_vec,  
            draw_stats: DrawStats::new(),  
        }
    }
    
    pub fn draw(&mut self, frame: &mut glium::Frame, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>) {
        self.draw_stats.reset();

        self.vert_vec.clear();
        self.index_vec.clear();
        
        for row in 0..self.squares.len() {
            for column in 0..self.squares[row].len() {
                
                if !self.squares[row][column].can_fit_in_buffers(Self::MAX_VERTICES, Self::MAX_INDICES, self.vert_vec.len(), self.index_vec.len()) {
                    self.empty(frame, world_transform, draw_parameters);
                }

                self.squares[row][column].add_to_ib_vec(&mut self.index_vec, self.vert_vec.len());
                self.squares[row][column].add_to_vb_vec(&mut self.vert_vec);
            }
        }
        
        if self.vert_vec.len() > 0{
            self.empty(frame, world_transform, draw_parameters);
        }
    }

    fn empty(&mut self, frame: &mut glium::Frame, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>) {
        self.draw_stats.indices += self.index_vec.len();
        self.draw_stats.vertices += self.vert_vec.len();
        self.draw_stats.draws += 1;

        self.vertex_buffers.slice(0..self.vert_vec.len()).expect("Invalid vertex range").write(&self.vert_vec);
        self.index_buffers.slice(0..self.index_vec.len()).expect("Invalid index range").write(&self.index_vec); 

        self.shader.draw(&(&self.vertex_buffers, &self.index_buffers), frame, world_transform, &Matrix::new(), draw_parameters);

        self.vertex_buffers.invalidate();
        self.index_buffers.invalidate();
        self.vert_vec.clear();
        self.index_vec.clear();
    }
}
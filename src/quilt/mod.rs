pub mod square;

use crate::render::material::{material_manager::{MaterialManager, MaterialType}};
use crate::render::matrix::{WorldTransform};
use crate::render::shape::Vertex;
use crate::render::material::{Material};
use crate::render::matrix::Matrix;
use crate::util::frame_timing::FrameTiming;
use square::Square;

use glium::{VertexBuffer, IndexBuffer};

#[allow(dead_code)]
pub struct Quilt {
    width: usize,
    height: usize,
    squares: Vec<Vec<Square>>,
    vert_vec: Vec<Vec<Vertex>>,
    index_vec: Vec<Vec<u32>>,
    vertex_buffers: Vec<VertexBuffer<Vertex>>,
    index_buffers: Vec<IndexBuffer<u32>>,
    pub shader: Box<dyn Material>,
    frame_timing: FrameTiming,
    needs_updated: bool,
}

impl Quilt {

    pub const MAX_BUF_SQUARES: usize = 1000; // max squares per buffer
    pub const NUM_BUFFERS: usize = 1;
    pub const MAX_SQUARES: usize = Self::MAX_BUF_SQUARES * Self::NUM_BUFFERS;

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

        let mut vertex_buffers: Vec<VertexBuffer<Vertex>> = Vec::with_capacity(Self::NUM_BUFFERS);

        for _i in 0..Self::NUM_BUFFERS {
            vertex_buffers.push(VertexBuffer::empty_dynamic(display, Self::MAX_BUF_SQUARES * Square::MAX_VERTICES).unwrap())
        }

        let mut index_buffers: Vec<IndexBuffer<u32>> = Vec::with_capacity(Self::NUM_BUFFERS);

        for _i in 0..Self::NUM_BUFFERS {
            index_buffers.push(IndexBuffer::empty_dynamic(display, glium::index::PrimitiveType::TrianglesList, Self::MAX_BUF_SQUARES * Square::MAX_INDICES).unwrap());
        }

        println!("Loaded buffers");

        let mut vert_vec = Vec::with_capacity(Self::NUM_BUFFERS);
        let mut index_vec = Vec::with_capacity(Self::NUM_BUFFERS);

        let num_squares = width * height;

        for _ in 0..Self::NUM_BUFFERS {
            vert_vec.push(Vec::with_capacity((num_squares/Self::NUM_BUFFERS + 1) * Square::MAX_VERTICES));
            index_vec.push(Vec::with_capacity((num_squares/Self::NUM_BUFFERS + 1) * Square::MAX_INDICES));
        }

        println!("Allocated vecs");

        Self {
            width,
            height,
            squares,
            vertex_buffers,
            index_buffers,
            shader: shaders.get_material(MaterialType::SolidColorMaterial).unwrap(),
            frame_timing: FrameTiming::new(),
            needs_updated: true,
            vert_vec,
            index_vec,    
        }
    }

    pub fn draw(&mut self, frame: &mut glium::Frame, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>) {
        self.frame_timing.update_frame_time();

        
        if self.needs_updated {

            let vert_vec = &mut self.vert_vec;
            let index_vec = &mut self.index_vec;

            let mut vec_index = 0;

            for i in 0..Self::NUM_BUFFERS {
                vert_vec[i].clear();
                index_vec[i].clear();
            }

            println!("It took {}ms to allocate", self.frame_timing.delta_frame_time().num_nanoseconds().unwrap() as f32 / 1_000_000.0);
            self.frame_timing.update_frame_time();

            
            for row in &mut self.squares {
                for square in row {

                    let mut index: Vec<u32> = square.index_buffer.clone();

                    for value in &mut index {
                        *value = *value + vert_vec[vec_index].len() as u32
                    }

                    let mut vert = square.vertex_buffer.clone();

                    index_vec[vec_index].append(&mut index);
                    vert_vec[vec_index].append(&mut vert);

                    vec_index = (vec_index + 1) % Self::NUM_BUFFERS;

                    // println!("num vertices: {}", square.vertex_count);
                    // println!("num indices: {}", square.index_count);
                }
            }
        
            for i in 0..Self::NUM_BUFFERS {
                let mut write = self.vertex_buffers[i].map_write();
                for j in 0..vert_vec[i].len() {
                    write.set(j, vert_vec[i][j]);
                }
                
                let mut write = self.index_buffers[i].map_write();
                for j in 0..index_vec[i].len() {
                    write.set(j, index_vec[i][j]);
                }
                
            }

        }

        self.needs_updated = true;

        println!("It took {}ms to update", self.frame_timing.delta_frame_time().num_nanoseconds().unwrap() as f32 / 1_000_000.0);
        self.frame_timing.update_frame_time();

        for i in 0..Self::NUM_BUFFERS {
            self.shader.draw(&(&self.vertex_buffers[i], &self.index_buffers[i]), frame, world_transform, &Matrix::new(), draw_parameters);
        }

        println!("It took {}ms to draw", self.frame_timing.delta_frame_time().num_nanoseconds().unwrap() as f32 / 1_000_000.0);

    }
}
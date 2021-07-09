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
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
    pub shader: Box<dyn Material>,
    frame_timing: FrameTiming,
    needs_updated: bool,
}

impl Quilt {

    pub const MAX_SQUARES: usize = 256;

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

        let vertex_buffer = VertexBuffer::empty_dynamic(display, Self::MAX_SQUARES * Square::MAX_VERTICES).unwrap();
        let index_buffer = IndexBuffer::empty_dynamic(display, glium::index::PrimitiveType::TrianglesList, Self::MAX_SQUARES * Square::MAX_INDICES).unwrap();

        Self {
            width,
            height,
            squares,
            vertex_buffer,
            index_buffer,
            shader: shaders.get_material(MaterialType::SolidColorMaterial).unwrap(),
            frame_timing: FrameTiming::new(),
            needs_updated: true,
        }
    }

    pub fn draw(&mut self, frame: &mut glium::Frame, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>) {
        // self.frame_timing.update_frame_time();

        if self.needs_updated {
            let mut vert_vec = Vec::with_capacity(Self::MAX_SQUARES * Square::MAX_VERTICES);
            let mut index_vec = Vec::with_capacity(Self::MAX_SQUARES * Square::MAX_INDICES);
            
            for row in &mut self.squares {
                for square in row {
                    for shape in &mut square.shapes {
                        let mut index: Vec<u32> = shape.shape.get_indices().into_iter().map(|value| value + vert_vec.len() as u32).collect();
                        index_vec.append(&mut index);
                        let mut vert = shape.shape.get_vertices();
                        vert_vec.append(&mut vert);
                    }
                }
            }
        
            {
                let mut write = self.vertex_buffer.map_write();
                for i in 0..vert_vec.len() {
                    write.set(i, vert_vec[i]);
                }
                
                let mut write = self.index_buffer.map_write();
                for i in 0..index_vec.len() {
                    write.set(i, index_vec[i]);
                }
            }
        }

        self.needs_updated = false;

        // println!("It took {}ms to update", self.frame_timing.delta_frame_time().num_nanoseconds().unwrap() as f32 / 1_000_000.0);
        // self.frame_timing.update_frame_time();
    
        self.shader.draw(&(&self.vertex_buffer, &self.index_buffer), frame, world_transform, &Matrix::new(), draw_parameters);
        // println!("It took {}ms to draw", self.frame_timing.delta_frame_time().num_nanoseconds().unwrap() as f32 / 1_000_000.0);

    }
}
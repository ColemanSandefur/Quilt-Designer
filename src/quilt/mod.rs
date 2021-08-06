pub mod brush;
pub mod block;

use crate::quilt::brush::*;
use crate::render::matrix::{WorldTransform};
use crate::render::shape::Vertex;
use crate::render::material::{SolidColorMaterial};
use crate::render::matrix::Matrix;
use crate::render::picker::{Picker, PickerEntry};
use block::Block;

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
    pub width: usize,
    pub height: usize,
    pub squares: Vec<Vec<Block>>,
    vert_vec: Vec<Vertex>,
    index_vec: Vec<u32>,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
    pub shader: SolidColorMaterial,
    needs_updated: bool,
    pub draw_stats: DrawStats,
}

impl Quilt {

    pub const INIT_VERTICES: usize = 6000;
    pub const INIT_INDICES: usize = Self::INIT_VERTICES * 4;

    pub fn new(display: &dyn glium::backend::Facade, width: usize, height: usize, picker: &mut Picker) -> Self {
        let mut squares = Vec::with_capacity(height);

        for r in 0..height {
            let mut row = Vec::with_capacity(width);

            for c in 0..width {
                let mut square = Block::new(r, c, picker);

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

        let vertex_buffer= VertexBuffer::empty_dynamic(display, Self::INIT_VERTICES).unwrap();
        let index_buffer = IndexBuffer::empty_dynamic(display, glium::index::PrimitiveType::TrianglesList, Self::INIT_INDICES).unwrap();

        println!("Loaded buffers");

        let vert_vec = Vec::with_capacity(Self::INIT_VERTICES);
        let index_vec = Vec::with_capacity(Self::INIT_INDICES);

        println!("Allocated vecs");

        Self {
            width,
            height,
            squares,
            vertex_buffer,
            index_buffer,
            shader: crate::render::material::material_manager::get_material_manager().get_solid_color_material(),
            needs_updated: true,
            vert_vec,
            index_vec,  
            draw_stats: DrawStats::new(),  
        }
    }
    
    pub fn draw(&mut self, frame: &mut impl glium::Surface, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>, picker: &mut Picker) {
        self.draw_stats.reset();

        picker.clear_surface(frame, self.vertex_buffer.get_context());
        
        if self.needs_updated {

            // Rebuild the buffers

            self.vertex_buffer.invalidate();
            self.index_buffer.invalidate();
            self.vert_vec.clear();
            self.index_vec.clear();
    
            let max_vertices = self.vertex_buffer.len();
            let max_indices = self.index_buffer.len();

            self.needs_updated = false;

            for row in 0..self.squares.len() {
                for column in 0..self.squares[row].len() {
                    
                    if !self.squares[row][column].can_fit_in_buffers(max_vertices, max_indices, self.vert_vec.len(), self.index_vec.len()) {
                        self.empty(frame, world_transform, draw_parameters, Some(picker));
                        self.vert_vec.clear();
                        self.index_vec.clear();

                        // marks the buffers as "needing updated" because we want to re-size the buffers and re-render everything to get it down to 1 draw call
                        self.needs_updated = true;
                    }
    
                    self.squares[row][column].add_to_ib_vec(&mut self.index_vec, self.vert_vec.len());
                    self.squares[row][column].add_to_vb_vec(&mut self.vert_vec);

                }
            }

            if self.vert_vec.len() > 0{
                self.empty(frame, world_transform, draw_parameters, Some(picker));
            }
            
            // Resize the buffer to reduce the number of draw calls needed
            if self.draw_stats.vertices > self.vertex_buffer.len() {
                println!("Resizing vertex buffer");

                // slightly oversizes the buffer to reduce the number of reallocations
                self.vertex_buffer = VertexBuffer::empty_dynamic(self.vertex_buffer.get_context(), (self.draw_stats.vertices as f32 * 1.1) as usize).unwrap();
                self.vert_vec = Vec::with_capacity(self.draw_stats.vertices);
            }
            
            // Resize the buffer to reduce the number of draw calls needed
            if self.draw_stats.indices > self.index_buffer.len() {
                println!("Resizing index buffer");

                // slightly oversizes the buffer to reduce the number of reallocations
                self.index_buffer = IndexBuffer::empty_dynamic(self.index_buffer.get_context(), glium::index::PrimitiveType::TrianglesList, (self.draw_stats.indices as f32 * 1.1) as usize).unwrap();
                self.index_vec = Vec::with_capacity(self.draw_stats.indices);
            }
        } else {
            
            // just display the current buffers

            self.draw_buffer(frame, world_transform, draw_parameters, Some(picker));
        }
    }

    fn empty(&mut self, frame: &mut impl glium::Surface, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>, picker: Option<&mut Picker>) {
        self.vertex_buffer.invalidate();
        self.index_buffer.invalidate();

        self.vertex_buffer.slice_mut(0..self.vert_vec.len()).expect("Invalid vertex range").write(&self.vert_vec);
        self.index_buffer.slice_mut(0..self.index_vec.len()).expect("Invalid index range").write(&self.index_vec); 

        // Really bad way to invalidate index buffer, calling invalidate doesn't seem to do anything
        if self.index_buffer.len() - self.index_vec.len() > 0 {

            let slice = self.index_buffer.slice(self.index_vec.len()..).expect("Invalid index range");
            let mut buffer: Vec<u32> = Vec::with_capacity(slice.len());
            for _ in 0..slice.len() {
                buffer.push(0);
            }
            slice.write(&buffer);
        }

        self.draw_buffer(frame, world_transform, draw_parameters, picker);
    }

    fn draw_buffer(&mut self, frame: &mut impl glium::Surface, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>, picker: Option<&mut Picker>) {
        self.draw_stats.indices += self.index_vec.len();
        self.draw_stats.vertices += self.vert_vec.len();
        self.draw_stats.draws += 1;

        self.shader.draw(&(&self.vertex_buffer, &self.index_buffer), frame, world_transform, &Matrix::new(), draw_parameters);

        if picker.is_some() {
            picker.unwrap().draw(self.vertex_buffer.get_context(), world_transform, &self.vertex_buffer, &self.index_buffer, draw_parameters);
        }
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn click(&mut self, entry: &PickerEntry, brush: &Brush, picker: &mut Picker) -> bool {

        if self.squares[entry.row][entry.column].click(entry.id, brush, picker) {
            self.needs_updated = true;

            return true;
        }


        false
    }
}
pub mod square;

use crate::render::material::{material_manager::{MaterialManager, MaterialType}};
use crate::render::matrix::{WorldTransform};
use crate::render::shape::Vertex;
use crate::render::material::{Material};
use crate::render::matrix::Matrix;
use crate::render::picker::{Picker, PickerEntry};
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
    pub width: usize,
    pub height: usize,
    pub squares: Vec<Vec<Square>>,
    vert_vec: Vec<Vertex>,
    index_vec: Vec<u32>,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
    pub shader: Box<dyn Material>,
    needs_updated: bool,
    pub draw_stats: DrawStats,
}

impl Quilt {

    pub const INIT_VERTICES: usize = 6000;
    pub const INIT_INDICES: usize = Self::INIT_VERTICES * 4;

    pub fn new(display: &dyn glium::backend::Facade, shaders: &mut MaterialManager, width: usize, height: usize, picker: &mut Picker) -> Self {

        let mut squares = Vec::with_capacity(height);

        for r in 0..height {
            let mut row = Vec::with_capacity(width);

            for c in 0..width {
                let mut square = Square::new(r, c, shaders, picker);

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
            shader: shaders.get_material(MaterialType::SolidColorMaterial).unwrap(),
            needs_updated: true,
            vert_vec,
            index_vec,  
            draw_stats: DrawStats::new(),  
        }
    }
    
    pub fn draw(&mut self, frame: &mut glium::Frame, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>, picker: &mut Picker) {
        self.draw_stats.reset();

        
        if self.needs_updated {
            // println!("Updating buffers");
            self.vert_vec.clear();
            self.index_vec.clear();
    
            let max_vertices = self.vertex_buffer.len();
            let max_indices = self.index_buffer.len();

            self.needs_updated = false;

            for row in 0..self.squares.len() {
                for column in 0..self.squares[row].len() {
                    
                    if !self.squares[row][column].can_fit_in_buffers(max_vertices, max_indices, self.vert_vec.len(), self.index_vec.len()) {
                        self.empty(frame, world_transform, draw_parameters);
                        self.vert_vec.clear();
                        self.index_vec.clear();
                    }
    
                    self.squares[row][column].add_to_ib_vec(&mut self.index_vec, self.vert_vec.len());
                    self.squares[row][column].add_to_vb_vec(&mut self.vert_vec);

                }
            }

            if self.vert_vec.len() > 0{
                self.empty(frame, world_transform, draw_parameters);
            }

            if self.draw_stats.vertices > self.vertex_buffer.len() {
                println!("Resizing vertex buffer");
                self.vertex_buffer = VertexBuffer::empty_dynamic(self.vertex_buffer.get_context(), self.draw_stats.vertices).unwrap();
                self.vert_vec = Vec::with_capacity(self.draw_stats.vertices);
                self.needs_updated = true;
            }
    
            if self.draw_stats.indices > self.index_buffer.len() {
                println!("Resizing index buffer");
                self.index_buffer = IndexBuffer::empty_dynamic(self.index_buffer.get_context(), glium::index::PrimitiveType::TrianglesList, self.draw_stats.indices).unwrap();
                self.index_vec = Vec::with_capacity(self.draw_stats.indices);
                self.needs_updated = true;
            }
        } else {
            self.draw_buffer(frame, world_transform, draw_parameters, Some(picker));
        }
    }

    fn empty(&mut self, frame: &mut glium::Frame, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>) {
        self.draw_stats.indices += self.index_vec.len();
        self.draw_stats.vertices += self.vert_vec.len();
        self.draw_stats.draws += 1;

        self.vertex_buffer.slice(0..self.vert_vec.len()).expect("Invalid vertex range").write(&self.vert_vec);
        self.index_buffer.slice(0..self.index_vec.len()).expect("Invalid index range").write(&self.index_vec); 

        self.draw_buffer(frame, world_transform, draw_parameters, None);

        self.vertex_buffer.invalidate();
        self.index_buffer.invalidate();
    }

    fn draw_buffer(&mut self, frame: &mut glium::Frame, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>, picker: Option<&mut Picker>) {
        self.draw_stats.indices += self.index_vec.len();
        self.draw_stats.vertices += self.vert_vec.len();
        self.draw_stats.draws += 1;

        self.shader.draw(&(&self.vertex_buffer, &self.index_buffer), frame, world_transform, &Matrix::new(), draw_parameters);

        if picker.is_some() {
            picker.unwrap().draw(frame, self.vertex_buffer.get_context(), world_transform, &self.vertex_buffer, &self.index_buffer, draw_parameters);
        }
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn click(&mut self, entry: &PickerEntry) -> bool {

        if self.squares[entry.row][entry.column].click(entry.id) {
            self.needs_updated = true;

            return true;
        }


        false
    }
}
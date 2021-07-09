use crate::render::object::{ShapeDataStruct};
use crate::render::material::{material_manager::{MaterialManager, MaterialType}};
use crate::render::matrix::{Matrix};
use crate::render::material::Material;
use crate::render::shape::Vertex;

pub struct Square {
    pub shapes: Vec<Box<ShapeDataStruct>>,
    model_transform: Matrix,
    pub vertex_buffer: Vec<Vertex>,
    pub index_buffer: Vec<u32>,
    pub shader: Box<dyn Material>,
}

impl Square {

    pub const MAX_VERTICES: usize = 1024;
    pub const MAX_INDICES: usize = 1024 * 4;

    pub fn new(shaders: &mut MaterialManager) -> Self {
        let mut shapes: Vec<Box<ShapeDataStruct>> = vec!{
            Box::new(ShapeDataStruct::new(
                Box::new(crate::render::shape::Square::with_width_height(0.0, 0.0, 1.0, 1.0)),
            )),
            Box::new(ShapeDataStruct::new(
                Box::new(crate::render::shape::Square::with_width_height(0.25, 0.25, 0.5, 0.5)),
            )),
        };

        shapes.get_mut(1).unwrap().shape.set_color([0.3, 0.3, 0.8, 1.0]);

        // let mut vertex_buffer = glium::VertexBuffer::empty_dynamic(display, 1024).unwrap();
        // let mut index_buffer = glium::IndexBuffer::empty_dynamic(display, glium::index::PrimitiveType::TrianglesList, 1024 * 4).unwrap();

        let mut vert_vec = Vec::with_capacity(Self::MAX_VERTICES);
        let mut index_vec = Vec::with_capacity(Self::MAX_INDICES);

        for shape in &mut shapes {
            let mut index: Vec<u32> = shape.shape.get_indices().into_iter().map(|value| value + vert_vec.len() as u32).collect();
            index_vec.append(&mut index);
            let mut vert = shape.shape.get_vertices();
            vert_vec.append(&mut vert);
        }


        let model_transform = Matrix::new();

        Self {
            shapes,
            model_transform,
            vertex_buffer: vert_vec,
            index_buffer: index_vec,
            shader: shaders.get_material(MaterialType::SolidColorMaterial).unwrap(),
        }
    }

    // pub fn draw(&mut self, frame: &mut glium::Frame, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>) {
    //     // for shape_data in &self.shapes {
    //     //     shape_data.shader.draw(&(&shape_data.shape.get_vertex_buffer(), &shape_data.shape.get_index_buffer()), frame, world_transform, &self.model_transform, draw_parameters);
    //     // }
        
    //     let mut vert_vec = Vec::with_capacity(1024);
    //     let mut index_vec = Vec::with_capacity(self.vertex_buffer.len());
    
    //     for shape in &mut self.shapes {
    //         shape.shape.set_model_matrix(&self.model_transform);
    //         let mut index: Vec<u32> = shape.shape.get_indices().into_iter().map(|value| value + vert_vec.len() as u32).collect();
    //         index_vec.append(&mut index);
    //         let mut vert = shape.shape.get_vertices();
    //         vert_vec.append(&mut vert);
    //     }
    
    //     {
    //         let mut write = self.vertex_buffer.map_write();
    //         for i in 0..vert_vec.len() {
    //             write.set(i, vert_vec[i]);
    //         }
            
    //         let mut write = self.index_buffer.map_write();
    //         for i in 0..index_vec.len() {
    //             write.set(i, index_vec[i]);
    //         }
    //     }
    
    //     println!("{:?}", index_vec);
    
    //     self.shader.draw(&(&self.vertex_buffer, &self.index_buffer), frame, world_transform, &self.model_transform, draw_parameters);
    //     // frame.draw()
    
    // }

    pub fn get_model_transform(&self) -> Matrix {
        self.model_transform.clone()
    }

    pub fn set_model_transform(&mut self, matrix: Matrix) {
        self.model_transform = matrix;

        for shape in &mut self.shapes {
            shape.shape.set_model_matrix(self.model_transform.clone());
        }
    }
}
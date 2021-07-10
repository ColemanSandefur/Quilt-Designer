use crate::render::object::{ShapeDataStruct};
use crate::render::material::{material_manager::{MaterialManager, MaterialType}};
use crate::render::matrix::{Matrix};
use crate::render::material::Material;
use crate::render::shape::Vertex;

use lyon::path::{ArcFlags, Path};
use lyon::path::builder::SvgPathBuilder;
use lyon::math::point;
use lyon::geom::vector;
use lyon::geom::Angle;

pub struct Square {
    pub shapes: Vec<Box<ShapeDataStruct>>,
    model_transform: Matrix,
    pub vertex_buffer: Vec<Vertex>,
    pub index_buffer: Vec<u32>,
    pub shader: Box<dyn Material>,
    pub index_count: usize,
    pub vertex_count: usize,
}

impl Square {

    pub const MAX_VERTICES: usize = 256;
    pub const MAX_INDICES: usize = Self::MAX_VERTICES * 4;

    pub fn new(shaders: &mut MaterialManager) -> Self {

        let mut half_circle = Path::svg_builder().flattened(0.0001);
        half_circle.move_to(point(0.5, 0.25));
        half_circle.relative_arc_to(
            vector(0.25, 0.25),
            Angle {radians: 3.14},
            ArcFlags {
                large_arc: true,
                sweep: true
            },
            vector(0.0, 0.5),
        );
        half_circle.close();
        let half_circle = half_circle.build();

        let mut shapes: Vec<Box<ShapeDataStruct>> = vec!{
            Box::new(ShapeDataStruct::new(
                Box::new(crate::render::shape::Square::with_width_height(0.0, 0.0, 1.0, 1.0)),
            )),
            Box::new(ShapeDataStruct::new(
                Box::new(crate::render::shape::Square::with_width_height(0.25, 0.25, 0.5, 0.5)),
            )),
            Box::new(ShapeDataStruct::new(
                Box::new(crate::render::shape::Square::with_width_height(0.3, 0.3, 0.4, 0.4)),
            )),
            Box::new(ShapeDataStruct::new(
                Box::new(crate::render::shape::Square::with_width_height(0.35, 0.35, 0.3, 0.3)),
            )),
            Box::new(ShapeDataStruct::new(
                Box::new(
                    crate::render::shape::PathShape::new(half_circle),
                ),
            )),
        };

        shapes.get_mut(1).unwrap().shape.set_color([0.3, 0.3, 0.8, 1.0]);
        shapes.get_mut(2).unwrap().shape.set_color([0.8, 0.2, 0.2, 1.0]);
        shapes.get_mut(3).unwrap().shape.set_color([0.1, 0.8, 0.8, 1.0]);

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

        let mut s = Self {
            shapes,
            model_transform,
            vertex_buffer: vert_vec,
            index_buffer: index_vec,
            shader: shaders.get_material(MaterialType::SolidColorMaterial).unwrap(),
            index_count: 0,
            vertex_count: 0,
        };

        s.update_vertex_index_counts();

        s
    }

    fn update_vertex_index_counts(&mut self) {
        self.index_count = 0;
        self.vertex_count = 0;
        for shape in &mut self.shapes {
            self.index_count += shape.shape.get_num_indices();
            self.vertex_count += shape.shape.get_num_vertices();
        }
    }

    fn update_buffer(&mut self) {
        self.update_vertex_index_counts();

        let mut vert_vec = Vec::with_capacity(self.vertex_count);
        let mut index_vec = Vec::with_capacity(self.index_count);

        for shape in &mut self.shapes {
            let mut index: Vec<u32> = shape.shape.get_indices().into_iter().map(|value| value + vert_vec.len() as u32).collect();
            index_vec.append(&mut index);
            let mut vert = shape.shape.get_vertices();
            vert_vec.append(&mut vert);
        }

        self.vertex_buffer = vert_vec;
        self.index_buffer = index_vec;
    }

    pub fn get_model_transform(&self) -> Matrix {
        self.model_transform.clone()
    }

    pub fn set_model_transform(&mut self, matrix: Matrix) {
        self.model_transform = matrix;

        for shape in &mut self.shapes {
            shape.shape.set_model_matrix(self.model_transform.clone());
        }

        self.update_buffer();
    }

    pub fn add_to_vb(&mut self, vertex_buffer: &mut glium::buffer::WriteMapping<[crate::render::shape::Vertex]>, index: &mut usize) {
        for i in 0..self.vertex_buffer.len() {
            let vb = self.vertex_buffer[i];
            vertex_buffer.set(*index + i, vb);
        }

        *index = *index + self.vertex_buffer.len();
    }

    pub fn add_to_ib(&mut self, index_buffer: &mut glium::buffer::WriteMapping<[u32]>, index: &mut usize) {
        for i in 0..self.index_buffer.len() {
            index_buffer.set(*index + i, self.index_buffer[i]);
        }

        *index = *index + self.vertex_buffer.len();
    }
}
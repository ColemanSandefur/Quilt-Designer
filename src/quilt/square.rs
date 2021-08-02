pub mod square_pattern;

use crate::quilt::brush::*;
use crate::render::object::{ShapeDataStruct};
use crate::render::matrix::{Matrix};
use crate::render::shape::{Shape, Vertex};
use crate::render::picker::{Picker};

use lyon::path::{ArcFlags, Path};
use lyon::path::builder::SvgPathBuilder;
use lyon::math::point;
use lyon::geom::vector;
use lyon::geom::Angle;

struct ShapeProtector {
    shapes: Vec<Box<ShapeDataStruct>>,
    model_transform: Matrix,
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<u32>,
    index_count: usize,
    vertex_count: usize,
}

#[allow(dead_code)]
impl ShapeProtector {

    // constructors

    pub fn new() -> Self {
        let s = Self {
            shapes: Vec::with_capacity(10),
            model_transform: Matrix::new(),
            vertex_buffer: Vec::new(),
            index_buffer: Vec::new(),
            index_count: 0,
            vertex_count: 0,
        };
        
        s
    }
    
    pub fn with_shapes(shapes: Vec<Box<ShapeDataStruct>>) -> Self {
        let mut s = Self {
            shapes,
            model_transform: Matrix::new(),
            vertex_buffer: Vec::new(),
            index_buffer: Vec::new(),
            index_count: 0,
            vertex_count: 0,
        };

        s.update_buffer();

        s
    }

    // shape accessors

    pub fn get_shapes(&self) -> &Vec<Box<ShapeDataStruct>> {
        &self.shapes
    }

    pub fn get_shape(&self, index: usize) -> Option<&Box<ShapeDataStruct>> {
        self.shapes.get(index)
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(Box::new(ShapeDataStruct::new(shape)));

        self.update_buffer();
    }

    pub fn remove_shape(&mut self, index: usize, picker: &mut Picker) {
        if let Some(shape_data) = self.shapes.get(index) {
            picker.remove_id(shape_data.shape.get_id());

            self.shapes.remove(index);

            self.update_buffer();
        }
    }

    pub fn set_shapes(&mut self, shapes: Vec<Box<ShapeDataStruct>>) {
        self.shapes = shapes;

        self.set_model_transform(self.model_transform);

        self.update_buffer();
    }

    // model accessors

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

    // index and vertex helpers

    pub fn get_index_count(&self) -> usize {
        self.index_count
    }

    pub fn get_vertex_count(&self) -> usize {
        self.vertex_count
    }

    pub fn get_vb(&self) -> &Vec<Vertex> {
        &self.vertex_buffer
    }

    pub fn get_ib(&self) -> &Vec<u32> {
        &self.index_buffer
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

    // copying local buffer to passed buffers

    pub fn add_to_vb_vec(&self, vertex_buffer: &mut Vec<crate::render::shape::Vertex>) {
        for i in 0..self.vertex_buffer.len() {
            vertex_buffer.push(self.vertex_buffer[i]);
        }
    }

    pub fn add_to_ib_vec(&self, index_buffer: &mut Vec<u32>, vb_index: usize) {
        for i in 0..self.index_buffer.len() {
            index_buffer.push(vb_index as u32 + self.index_buffer[i]);
        }
    }

    pub fn can_fit_in_buffers(&self, vb_capacity: usize, ib_capacity: usize, vb_index: usize, ib_index: usize) -> bool {
        vb_index + self.vertex_buffer.len() <= vb_capacity && ib_index + self.index_buffer.len() <= ib_capacity
    }

    // shape modification

    pub fn set_shape_color(&mut self, index: usize, color: [f32; 4]) {
        if let Some(shape) = self.shapes.get_mut(index) {
            shape.shape.set_color(color);

            self.update_buffer();
        }
    }
}

#[allow(dead_code)]
pub struct Square {
    shape_protector: ShapeProtector,
    row: usize,
    column: usize,
}

impl Square {

    pub const MAX_VERTICES: usize = 256;
    pub const MAX_INDICES: usize = Self::MAX_VERTICES * 4;

    pub fn new(row: usize, column: usize, picker: &mut Picker) -> Self {

        let mut half_circle = Path::svg_builder().flattened(0.001);
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

        let mut shape_protector = ShapeProtector::with_shapes(
            vec!{
                Box::new(ShapeDataStruct::new(
                    Box::new(crate::render::shape::Square::with_width_height(0.0, 0.0, 1.0, 1.0, 0)),
                )),
                Box::new(ShapeDataStruct::new(
                    Box::new(crate::render::shape::Square::with_width_height(0.05, 0.05, 0.9, 0.9, picker.get_new_id(row, column))),
                )),
                Box::new(ShapeDataStruct::new(
                    Box::new(crate::render::shape::Square::with_width_height(0.25, 0.25, 0.5, 0.5, picker.get_new_id(row, column))),
                )),
                Box::new(ShapeDataStruct::new(
                    Box::new(crate::render::shape::Square::with_width_height(0.3, 0.3, 0.4, 0.4, picker.get_new_id(row, column))),
                )),
                Box::new(ShapeDataStruct::new(
                    Box::new(crate::render::shape::Square::with_width_height(0.35, 0.35, 0.3, 0.3, picker.get_new_id(row, column))),
                )),
                Box::new(ShapeDataStruct::new(
                    Box::new(
                        crate::render::shape::PathShape::new(half_circle, picker.get_new_id(row, column)),
                    ),
                )),
            }
        );

        shape_protector.set_shape_color(0, [0.0, 0.0, 0.0, 1.0]);
        shape_protector.set_shape_color(2, [0.3, 0.3, 0.8, 1.0]);
        shape_protector.set_shape_color(3, [0.8, 0.2, 0.2, 1.0]);
        shape_protector.set_shape_color(4, [0.1, 0.8, 0.8, 1.0]);

        let s = Self {
            shape_protector,
            row,
            column,
        };

        s
    }

    pub fn add_to_vb_vec(&self, vertex_buffer: &mut Vec<crate::render::shape::Vertex>) {
        self.shape_protector.add_to_vb_vec(vertex_buffer);
    }

    pub fn add_to_ib_vec(&self, index_buffer: &mut Vec<u32>, vb_index: usize) {
        self.shape_protector.add_to_ib_vec(index_buffer, vb_index);
    }

    pub fn can_fit_in_buffers(&self, vb_capacity: usize, ib_capacity: usize, vb_index: usize, ib_index: usize) -> bool {
        self.shape_protector.can_fit_in_buffers(vb_capacity, ib_capacity, vb_index, ib_index)
    }

    //returns wether or not it clicked
    pub fn click(&mut self, id: u32, brush: &Brush, picker: &mut Picker) -> bool {
        let mut was_clicked = false;
        
        for index in 0..self.shape_protector.get_shapes().len() {
            
            if self.shape_protector.get_shape(index).unwrap().shape.was_clicked(id) {
                if brush.is_pattern_brush() {
                    self.shape_protector.set_shape_color(index, brush.get_pattern_brush().unwrap().color);
                } else {
                    self.shape_protector.set_shapes(brush.get_block_brush().unwrap().get_pattern(picker, self.row, self.column).get_shapes().clone());
                }

                was_clicked = true;

                break;
            }
        }

        was_clicked
    }

    pub fn set_shape_color(&mut self, index: usize, color: [f32; 4]) {
        self.shape_protector.set_shape_color(index, color);
    }

    pub fn get_model_transform(&self) -> Matrix {
        self.shape_protector.get_model_transform()
    }

    pub fn set_model_transform(&mut self, matrix: Matrix) {
        self.shape_protector.set_model_transform(matrix);
    }
}
pub mod block_pattern;
pub mod block_manager;

use crate::quilt::brush::*;
use crate::render::shape_object::{ShapeDataStruct};
use crate::render::matrix::{Matrix};
use crate::render::shape::{Shape, Vertex};
use crate::render::picker::{Picker};
use crate::quilt::block::block_pattern::BlockPattern;
use crate::parse::*;

// The purpose of the "shape protector" is to call update_buffer whenever a shape has changed
struct ShapeProtector {
    shapes: Vec<Box<ShapeDataStruct>>,
    rotation: f32,
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
            rotation: 0.0,
            model_transform: Matrix::new(),
            vertex_buffer: Vec::new(),
            index_buffer: Vec::new(),
            index_count: 0,
            vertex_count: 0,
        };
        
        s
    }
    
    pub fn with_shapes(mut shapes: Vec<Box<ShapeDataStruct>>, rotation: f32) -> Self {

        for shape in &mut shapes {
            shape.shape.set_rotation(rotation);
        }

        let mut s = Self {
            shapes,
            rotation,
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

    pub fn set_shapes(&mut self, shapes: Vec<Box<ShapeDataStruct>>, rotation: f32) {
        self.shapes = shapes;

        self.set_model_transform(self.model_transform);
        self.rotation = rotation;

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

    pub fn apply_brush(&mut self, index: usize, pattern_brush: &crate::quilt::brush::PatternBrush) {

        if let Some(shape) = self.shapes.get_mut(index) {
            pattern_brush.apply_to_shape(shape);

            self.update_buffer();
        }

    }

    pub fn set_rotation(&mut self, rotation: f32) {
        for shape in &mut self.shapes {
            shape.shape.set_rotation(rotation);
        }
    }

    // serialization

    pub fn from_save(yaml: &Yaml, picker: &mut Picker, row: usize, column: usize) -> Self {
        let yaml_map = LinkedHashMap::from(yaml);

        let yaml_vec: Vec<Yaml> = yaml_map.get("shape").into();
        
        let mut shapes: Vec<Box<ShapeDataStruct>> = yaml_vec.into_iter().map(|data| ShapeDataStruct::from_save(data)).collect();

        for shape in &mut shapes {
            shape.shape.set_id(picker.get_new_id(row, column));
            shape.shape.set_color([1.0;4]);
        }

        BlockPattern::apply_background(&mut shapes);

        shapes[0].shape.set_id(picker.get_new_id(row, column));
        shapes[0].shape.set_color([1.0; 4]);

        println!("shapes len: {}", shapes.len());
        
        let mut s = Self {
            shapes,
            rotation: 0.0,
            model_transform: Matrix::new(),
            vertex_buffer: Vec::new(),
            index_buffer: Vec::new(),
            index_count: 0,
            vertex_count: 0,
        };

        // s.set_rotation(yaml_map.get("rotation").into());

        s.update_buffer();

        println!("Num indices: {}", s.index_count);

        s
    }

    pub fn to_save(&self) -> Yaml {
        // IMPORTANT: assuming that the first and last shapes are redundant (background square and border)

        let mut vec: Vec<Yaml> = Vec::with_capacity(self.shapes.len());

        if self.shapes.len() > 2 {
            for shape in &self.shapes[1..self.shapes.len() - 1] {
                vec.push(shape.to_save());
            }
        }

        LinkedHashMap::create(vec![
            ("shape", Yaml::from(vec)),
            ("rotation", self.rotation.into())
        ])
    }
}

// Each square represents a block on the quilt

#[allow(dead_code)]
pub struct Block {
    shape_protector: ShapeProtector,
    row: usize,
    column: usize,
}

impl Block {
    // Determines how thick the boarders are for the shapes
    pub const BLOCK_BORDER_WIDTH: f32 = 0.05;
    pub const SHAPE_BORDER_WIDTH: f32 = 0.02;

    pub fn new(row: usize, column: usize, picker: &mut Picker) -> Self {

        // the default pattern for the block
        let shape_protector = ShapeProtector::with_shapes(
            vec!{
                Box::new(ShapeDataStruct::new(
                    Box::new(crate::render::shape::PathShape::square_with_line_width(0.0, 0.0, 1.0, 1.0, picker.get_new_id(row, column), 0.0)),
                )),
                Box::new(ShapeDataStruct::new(
                    Box::new(crate::render::shape::StrokeShape::square(0.0, 0.0, 1.0, 1.0, 0, &lyon::tessellation::StrokeOptions::default().with_line_width(crate::quilt::block::Block::BLOCK_BORDER_WIDTH))),
                )),
            },
            0.0
        );

        let s = Block {
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
        let mut should_update = false;

        if brush.is_block_brush() {
            // change the block pattern
            self.shape_protector.set_shapes(brush.get_block_brush().unwrap().get_pattern(picker, self.row, self.column).get_shapes().clone(), Brush::get_rotation());
            should_update = true;
        } else if brush.is_pattern_brush() {
            // change either the color or texture of a shape

            // find which shape was clicked
            for index in 0..self.shape_protector.get_shapes().len() {
                if self.shape_protector.get_shape(index).unwrap().shape.was_clicked(id) {
                    self.shape_protector.apply_brush(index, &brush.get_pattern_brush().unwrap());
    
                    should_update = true;
    
                    break;
                }
            }
        }
        

        should_update
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

    pub fn from_save(yaml:Yaml, picker: &mut Picker) -> Self {
        let map = LinkedHashMap::from(yaml);

        let row = usize::from(map.get("row"));
        let column = usize::from(map.get("column"));

        let shape_protector = ShapeProtector::from_save(map.get("shapes"), picker, row, column);

        Self {
            shape_protector,
            row,
            column
        }
    }

    pub fn to_save(&self) -> Yaml {
        let shapes = self.shape_protector.to_save();

        LinkedHashMap::create(vec![
            ("shapes", Yaml::from(shapes)),
            ("row", self.row.into()),
            ("column", self.column.into()),
        ])
    }
}
use crate::render::object::{ShapeDataStruct};

#[derive(Clone)]
pub struct SquarePattern {
    shapes: Vec<Box<ShapeDataStruct>>,
}

impl SquarePattern {
    pub fn new(shapes: Vec<Box<ShapeDataStruct>>) -> Self {
        Self {
            shapes,
        }
    }
    pub fn get_shapes(&self) -> &Vec<Box<ShapeDataStruct>> {
        &self.shapes
    }
}

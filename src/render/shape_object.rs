use crate::parse::{Yaml, SavableBlueprint};
use crate::render::matrix::{Matrix};
use crate::render::shape::{Shape, PathShape};

// Everything rendered will be a Shape Object, this will be added to the renderer's list
// the renderer will then handle the drawing of the object

pub struct ShapeDataStruct {
    pub shape: Box<dyn Shape>,
}

impl ShapeDataStruct {
    pub fn new(shape: Box<dyn Shape>) -> Self {
        Self {
            shape,
        }
    }
}

impl Clone for ShapeDataStruct {
    fn clone(&self) -> Self {
        Self {
            shape: self.shape.clone_shape()
        }
    }
}

pub trait ShapeObject {
    fn get_shapes(&self) -> &Vec<Box<ShapeDataStruct>>;
    fn get_shapes_mut(&mut self) -> &mut Vec<Box<ShapeDataStruct>>;
    fn get_model_transform(&self) -> &Matrix;
    fn get_model_transform_mut(&mut self) -> &mut Matrix;
}

impl SavableBlueprint for ShapeDataStruct {
    fn to_save_blueprint(&self) -> Yaml {
        self.shape.to_save_blueprint()
    }

    fn from_save_blueprint(yaml: Yaml) -> Box<Self> where Self: Sized {
        Box::new(Self {
            shape: PathShape::from_save_blueprint(yaml)
        })
    }
}
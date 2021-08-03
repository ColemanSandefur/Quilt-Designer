
use crate::render::matrix::{Matrix};
use crate::render::shape::Shape;

use lyon::path::{ArcFlags, Path};
use lyon::path::builder::SvgPathBuilder;
use lyon::math::point;
use lyon::geom::vector;
use lyon::geom::Angle;

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

pub struct DefaultShapeObject {
    shapes: Vec<Box<ShapeDataStruct>>,
    model_transform: Matrix,
}

impl DefaultShapeObject {
    pub fn new() -> Self {

        let mut half_circle = Path::svg_builder().flattened(0.0001);
        half_circle.move_to(point(0.0, -0.25));
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

        let shapes: Vec<Box<ShapeDataStruct>> = vec!{
            Box::new(ShapeDataStruct::new(
                Box::new(crate::render::shape::Square::with_width_height(-0.25, -0.25, 0.5, 0.5, 0)),
            )),
            Box::new(ShapeDataStruct::new(
                Box::new(
                    crate::render::shape::PathShape::new(half_circle, 0),
                ),
            ))
        };

        let model_transform = Matrix::new();

        Self {
            shapes,
            model_transform,
        }
    }

    
}

impl ShapeObject for DefaultShapeObject {
    fn get_shapes(&self) -> &Vec<Box<ShapeDataStruct>> {
        &self.shapes
    }
    fn get_shapes_mut(&mut self) -> &mut Vec<Box<ShapeDataStruct>> {
        &mut self.shapes
    }
    fn get_model_transform(&self) -> &Matrix {
        &self.model_transform
    }
    fn get_model_transform_mut(&mut self) -> &mut Matrix {
        &mut self.model_transform
    }
}
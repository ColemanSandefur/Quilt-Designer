use crate::render::object::{ShapeObject, ShapeDataStruct};
use crate::render::material::{material_manager::{MaterialManager, MaterialType}};
use crate::render::matrix::{Matrix, WorldTransform};

pub struct Square {
    shapes: Vec<Box<ShapeDataStruct>>,
    pub model_transform: Matrix,
}

impl Square {
    pub fn new(display: &dyn glium::backend::Facade, shaders: &MaterialManager) -> Self {
        let shapes: Vec<Box<ShapeDataStruct>> = vec!{
            Box::new(ShapeDataStruct::new(
                Box::new(crate::render::shape::Square::with_width_height(display, 0.0, 0.0, 1.0, 1.0)), 
                shaders.get_material(MaterialType::SolidColorMaterial).unwrap(),
            )),
        };

        let model_transform = Matrix::new();

        Self {
            shapes,
            model_transform,
        }
    }
}

impl ShapeObject for Square {
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

    fn draw(&mut self, frame: &mut glium::Frame, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>) {
        for shape_data in &self.shapes {
            // crate::shapes::draw(&shape_data.shape, frame, program, &uniforms.add("local", shape_data.local_transform).add("color", shape_data.color), draw_parameters);
            shape_data.shader.draw(&shape_data.shape, frame, world_transform, &self.model_transform, draw_parameters);
        }
    }
}
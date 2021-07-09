use crate::render::object::{ShapeObject, ShapeDataStruct};
use crate::render::material::{material_manager::{MaterialManager, MaterialType}, MaterialHelper};
use crate::render::matrix::{Matrix, WorldTransform};
use crate::render::material::Material;

pub struct Square {
    pub shapes: Vec<Box<ShapeDataStruct>>,
    pub model_transform: Matrix,
}

impl Square {
    pub fn new(display: &dyn glium::backend::Facade, shaders: &mut MaterialManager) -> Self {
        let shapes: Vec<Box<ShapeDataStruct>> = vec!{
            Box::new(ShapeDataStruct::new(
                Box::new(crate::render::shape::Square::with_width_height(display, 0.0, 0.0, 1.0, 1.0)), 
                shaders.get_material(MaterialType::SolidColorMaterial).unwrap(),
                shaders.get_click_material()
            )),
            Box::new(ShapeDataStruct::new(
                Box::new(crate::render::shape::Square::with_width_height(display, 0.25, 0.25, 0.5, 0.5)), 
                Box::new(MaterialHelper::get_material::<crate::render::material::SolidColorMaterial>(&mut shaders.get_material(MaterialType::SolidColorMaterial).unwrap()).unwrap()
                    .create_from_existing([0.3, 0.3, 1.0, 1.0])),
                shaders.get_click_material()
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
            shape_data.shader.draw(&shape_data.shape, frame, world_transform, &self.model_transform, draw_parameters);
        }
    }

    fn draw_click(&mut self, frame: &mut glium::Frame, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>) {
        for shape_data in &self.shapes {
            shape_data.click_shader.draw(&shape_data.shape, frame, world_transform, &self.model_transform, draw_parameters);
        }
    }
}
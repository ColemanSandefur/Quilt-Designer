pub mod material_manager;

use crate::render::matrix::{Matrix, WorldTransform};
use crate::render::shape::Shape;
use material_manager::MaterialType;

use std::rc::Rc;

pub trait Material {
    fn clone_material(&self) -> Box<dyn Material>;
    fn as_any(&self) -> Box<&dyn std::any::Any>;
    fn as_any_mut(&mut self) -> Box<&mut dyn std::any::Any>;
    fn to_any(self) -> Box<dyn std::any::Any>;
    fn draw(&self, shape: &Box<dyn Shape>, surface: &mut glium::Frame, world_transform: &WorldTransform, model_transform: &Matrix, draw_parameters: &glium::DrawParameters<'_>);
    fn get_shader_type(&self) -> MaterialType;
}

pub struct SolidColorMaterial {
    pub shader: Rc<glium::Program>,
    pub color: [f32; 4],
}

impl SolidColorMaterial {
    pub fn new(shader: Rc<glium::Program>, color: [f32;4]) -> Self {
        Self {
            shader,
            color,
        }
    }

    pub fn create_from_existing(&self, new_color: [f32; 4]) -> Self {
        Self {
            shader: self.shader.clone(),
            color: new_color,
        }
    }
}

impl Material for SolidColorMaterial {
    fn clone_material(&self) -> Box<dyn Material> {
        Box::new(Self {
            shader: self.shader.clone(),
            color: self.color.clone()
        })
    }
    
    fn as_any(&self) -> Box<&dyn std::any::Any> {
        Box::new(self)
    }
    fn as_any_mut(&mut self) -> Box<&mut dyn std::any::Any> {
        Box::new(self)
    }
    fn to_any(self) -> Box<dyn std::any::Any> {
        Box::new(self)
    }
    fn draw(&self, shape: &Box<dyn Shape>, surface: &mut glium::Frame, world_transform: &WorldTransform, model_transform: &Matrix, draw_parameters: &glium::DrawParameters<'_>) {
        let uniforms = world_transform.to_uniform()
            .add("model", model_transform.get_matrix())
            .add("color", self.color);
        
        crate::render::shape::draw(shape, surface, &self.shader, &uniforms, draw_parameters);

        let mut new_color = self.color;
        new_color[0] -= 0.1;
        new_color[1] -= 0.1;
        new_color[2] -= 0.1;

        let uniforms = world_transform.to_uniform()
            .add("model", model_transform.get_matrix())
            .add("color", new_color);

        crate::render::shape::draw(shape, surface, &self.shader, &uniforms, &glium::DrawParameters {
            polygon_mode: glium::PolygonMode::Line,
            line_width: Some(3.0),
            multisampling: true,
            .. Default::default()
        })
    }

    fn get_shader_type(&self) -> MaterialType {
        MaterialType::SolidColorMaterial
    }
}

#[derive(Clone)]
pub struct ClickMaterial {
    pub shader: Rc<glium::Program>,
    pub color: [f32; 4],
}

impl ClickMaterial {
    pub fn new(shader: Rc<glium::Program>, color: [f32;4]) -> Self {
        Self {
            shader,
            color,
        }
    }

    pub fn create_from_existing(&self, new_color: [f32; 4]) -> Self {
        Self {
            shader: self.shader.clone(),
            color: new_color,
        }
    }
}

impl Material for ClickMaterial {
    fn clone_material(&self) -> Box<dyn Material> {
        Box::new(Self {
            shader: self.shader.clone(),
            color: self.color.clone()
        })
    }
    
    fn as_any(&self) -> Box<&dyn std::any::Any> {
        Box::new(self)
    }
    fn as_any_mut(&mut self) -> Box<&mut dyn std::any::Any> {
        Box::new(self)
    }
    fn to_any(self) -> Box<dyn std::any::Any> {
        Box::new(self)
    }
    fn draw(&self, shape: &Box<dyn Shape>, surface: &mut glium::Frame, world_transform: &WorldTransform, model_transform: &Matrix, draw_parameters: &glium::DrawParameters<'_>) {
        let uniforms = world_transform.to_uniform()
            .add("model", model_transform.get_matrix())
            .add("color", self.color);
        
        crate::render::shape::draw(shape, surface, &self.shader, &uniforms, draw_parameters);

        let mut new_color = self.color;
        new_color[0] -= 0.1;
        new_color[1] -= 0.1;
        new_color[2] -= 0.1;

        let uniforms = world_transform.to_uniform()
            .add("model", model_transform.get_matrix())
            .add("color", new_color);

        crate::render::shape::draw(shape, surface, &self.shader, &uniforms, &glium::DrawParameters {
            polygon_mode: glium::PolygonMode::Line,
            line_width: Some(3.0),
            multisampling: true,
            .. Default::default()
        })
    }

    fn get_shader_type(&self) -> MaterialType {
        MaterialType::ClickMaterial
    }
}

pub struct MaterialHelper {}

impl MaterialHelper {
    pub fn get_material<T: std::any::Any>(material: &mut Box<dyn Material>) -> Option<&mut T> {
        material.as_any_mut().downcast_mut::<T>()
    }
}
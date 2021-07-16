pub mod material_manager;

use crate::render::matrix::{Matrix, WorldTransform};
use crate::render::shape::Vertex;
use material_manager::MaterialType;

use std::rc::Rc;

#[derive(Clone)]
pub struct SolidColorMaterial {
    pub shader: Rc<glium::Program>,
}

impl SolidColorMaterial {
    pub fn new(shader: Rc<glium::Program>) -> Self {
        Self {
            shader,
        }
    }

    pub fn create_from_existing(&self) -> Self {
        Self {
            shader: self.shader.clone(),
        }
    }
    
    pub fn as_any(&self) -> Box<&dyn std::any::Any> {
        Box::new(self)
    }

    pub fn as_any_mut(&mut self) -> Box<&mut dyn std::any::Any> {
        Box::new(self)
    }

    pub fn to_any(self) -> Box<dyn std::any::Any> {
        Box::new(self)
    }

    pub fn draw(&self, shape: &(&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>), surface: &mut impl glium::Surface, world_transform: &WorldTransform, _model_transform: &Matrix, draw_parameters: &glium::DrawParameters<'_>) {
        let uniforms = world_transform.to_uniform();
        
        crate::render::shape::draw(shape, surface, &self.shader, &uniforms, draw_parameters);
    }

    pub fn get_shader_type(&self) -> MaterialType {
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
    
    pub fn as_any(&self) -> Box<&dyn std::any::Any> {
        Box::new(self)
    }

    pub fn as_any_mut(&mut self) -> Box<&mut dyn std::any::Any> {
        Box::new(self)
    }

    pub fn to_any(self) -> Box<dyn std::any::Any> {
        Box::new(self)
    }

    pub fn draw(&self, shape: &(&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>), surface: &mut impl glium::Surface, world_transform: &WorldTransform, _model_transform: &Matrix, draw_parameters: &glium::DrawParameters<'_>) {
        let uniforms = world_transform.to_uniform();
        
        crate::render::shape::draw(shape, surface, &self.shader, &uniforms, draw_parameters);
    }

    pub fn get_shader_type(&self) -> MaterialType {
        MaterialType::ClickMaterial
    }
}
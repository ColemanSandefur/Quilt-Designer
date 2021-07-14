pub mod material_manager;

use crate::render::matrix::{Matrix, WorldTransform};
use crate::render::shape::Vertex;
use material_manager::MaterialType;

use std::rc::Rc;

pub trait Material {
    fn clone_material(&self) -> Box<dyn Material>;
    fn as_any(&self) -> Box<&dyn std::any::Any>;
    fn as_any_mut(&mut self) -> Box<&mut dyn std::any::Any>;
    fn to_any(self) -> Box<dyn std::any::Any>;
    fn draw(&self, shape: &(&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>), surface: &mut glium::Frame, world_transform: &WorldTransform, model_transform: &Matrix, draw_parameters: &glium::DrawParameters<'_>);
    fn draw_frame_buffer(&self, shape: &(&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>), surface: &mut glium::framebuffer::SimpleFrameBuffer, world_transform: &WorldTransform, model_transform: &Matrix, draw_parameters: &glium::DrawParameters<'_>);
    fn get_shader_type(&self) -> MaterialType;
}

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
}

impl Material for SolidColorMaterial {
    fn clone_material(&self) -> Box<dyn Material> {
        Box::new(Self {
            shader: self.shader.clone(),
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
    fn draw(&self, shape: &(&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>), surface: &mut glium::Frame, world_transform: &WorldTransform, _model_transform: &Matrix, draw_parameters: &glium::DrawParameters<'_>) {
        let uniforms = world_transform.to_uniform();
        
        crate::render::shape::draw(shape, surface, &self.shader, &uniforms, draw_parameters);
    }

    fn draw_frame_buffer(&self, shape: &(&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>), surface: &mut glium::framebuffer::SimpleFrameBuffer, world_transform: &WorldTransform, _model_transform: &Matrix, draw_parameters: &glium::DrawParameters<'_>) {
        let uniforms = world_transform.to_uniform();
        
        crate::render::shape::draw_frame_buffer(shape, surface, &self.shader, &uniforms, draw_parameters);
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
    fn draw(&self, shape: &(&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>), surface: &mut glium::Frame, world_transform: &WorldTransform, _model_transform: &Matrix, draw_parameters: &glium::DrawParameters<'_>) {
        let uniforms = world_transform.to_uniform();
        
        crate::render::shape::draw(shape, surface, &self.shader, &uniforms, draw_parameters);
    }

    fn draw_frame_buffer(&self, shape: &(&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>), surface: &mut glium::framebuffer::SimpleFrameBuffer, world_transform: &WorldTransform, _model_transform: &Matrix, draw_parameters: &glium::DrawParameters<'_>) {
        let uniforms = world_transform.to_uniform();
        
        crate::render::shape::draw_frame_buffer(shape, surface, &self.shader, &uniforms, draw_parameters);
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
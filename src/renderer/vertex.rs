

use cgmath::Matrix4;
use cgmath::Rad;
use cgmath::prelude::*;
use lyon::math::{point, Point};

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub model: [[f32;4]; 4],
    pub rotation: [[f32;4]; 4],
    pub id: u32,
    pub tex_id: u32,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: [0.0; 2],
            color: [1.0; 4],
            model: Matrix4::from_diagonal([1.0; 4].into()).into(),
            rotation: Matrix4::from_angle_z(Rad(0.0)).into(),
            id: 0,
            tex_id: 0,
        }
    }
}

impl Vertex {
    pub fn to_point(&self) -> Point {
        point(self.position[0], self.position[1])
    }
}

implement_vertex!(Vertex, position, color, model, rotation, id, tex_id);
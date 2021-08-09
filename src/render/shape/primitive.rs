use super::*;
use crate::render::matrix::Matrix;
use cgmath::Matrix4;

use lyon::math::{point};

#[derive(Clone)]
pub struct Triangle {
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<u32>,
}

impl Triangle {
    pub fn new(pos1: (f32, f32), pos2: (f32, f32), pos3: (f32, f32), id: u32) -> Self {
        let mut vertex_buffer = vec!{
            Vertex { position: [pos1.0, pos1.1], id, .. Default::default() },
            Vertex { position: [pos2.0, pos2.1], id, .. Default::default() },
            Vertex { position: [pos3.0, pos3.1], id, .. Default::default() },
        };

        let mut index_buffer = vec!{0u32, 1, 2};

        // Generate outline

        let mut outline = ShapePath::new();
        outline.move_to(point(pos1.0, pos1.1));
        outline.line_to(point(pos2.0, pos2.1));
        outline.line_to(point(pos3.0, pos3.1));
        outline.line_to(point(pos1.0, pos1.1));
        outline.close();

        let stroke = StrokeShape::new(outline, 0, &StrokeOptions::default().with_line_width(crate::quilt::block::Block::SHAPE_BORDER_WIDTH));

        // Add generated ib and vb to current ib and vb

        for index in stroke.get_indices() {
            index_buffer.push(index + vertex_buffer.len() as u32);
        }

        for vertex in stroke.get_vertices() {
            vertex_buffer.push(vertex);
        }

        Self {
            vertex_buffer,
            index_buffer,
        }
    }
}

impl PrimitiveShape for Triangle {
    fn get_vertices(&self) -> Vec<Vertex> {
        self.vertex_buffer.clone()
    }

    fn get_indices(&self) -> Vec<u32> {
        self.index_buffer.clone()
    }

    fn set_color(&mut self, color: [f32; 4]) {
        // only change the color of the triangle, not the outline
        for vertex in &mut self.vertex_buffer[0..3] {
            vertex.color = color;
        }
    }

    fn set_model_matrix(&mut self, matrix: Matrix) {
        for vertex in &mut self.vertex_buffer {
            vertex.model = matrix.get_matrix();
        }
    }

    fn get_num_vertices(&self) -> usize {
        self.vertex_buffer.len()
    }
    
    fn get_num_indices(&self) -> usize {
        self.index_buffer.len()
    }

    fn get_id(&self) -> u32 {
        match self.vertex_buffer.get(0) {
            Some(vertex) => vertex.id,
            None => 0,
        }
    }

    fn set_id(&mut self, id: u32) {
        // only change the id of the triangle not its outline
        for vertex in &mut self.vertex_buffer[0..3] {
            vertex.id = id;
        }
    }

    fn get_tex_id(&self) -> u32 {
        match self.vertex_buffer.get(0) {
            Some(vertex) => vertex.tex_id,
            None => 0,
        }
    }

    fn set_tex_id(&mut self, id: u32) {
        // only change the tex_id of the triangle not its outline
        for vertex in &mut self.vertex_buffer[0..3] {
            vertex.tex_id = id;
        }
    }

    fn set_rotation(&mut self, rotation: f32) {
        for vertex in &mut self.vertex_buffer {
            vertex.rotation = Matrix4::from_angle_z(cgmath::Rad(rotation)).into()
        }
    }

    fn clone_primitive(&self) -> Box<dyn PrimitiveShape> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct Square {
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<u32>,
}

impl Square {
    pub fn with_width_height(x: f32, y: f32, width: f32, height: f32, id: u32) -> Self {
        Square::with_line_width(x, y, width, height, id, crate::quilt::block::Block::SHAPE_BORDER_WIDTH)
    }

    pub fn with_line_width(x: f32, y:f32, width: f32, height: f32, id: u32, outline_width: f32) -> Self {
        let mut vertex_buffer = vec!{
            Vertex { position: [ x, y ], id, .. Default::default() },
            Vertex { position: [ x + width, y ], id, .. Default::default() },
            Vertex { position: [ x, y + height ], id, .. Default::default() },
            Vertex { position: [ x + width, y + height ], id, .. Default::default() },
        };

        let mut index_buffer = vec!{0u32, 1, 2, 1, 2, 3};
        
        // Generate outline

        let stroke = StrokeShape::square(x, y, width, height, 0, &StrokeOptions::default().with_line_width(outline_width));

        // Join stroke vb and ib to current ib and vb

        for index in stroke.get_indices() {
            index_buffer.push(index + vertex_buffer.len() as u32);
        }

        for vertex in stroke.get_vertices() {
            vertex_buffer.push(vertex);
        }

        Self {
            vertex_buffer,
            index_buffer,
        }
    }
}

impl PrimitiveShape for Square {
    fn get_vertices(&self) -> Vec<Vertex> {
        self.vertex_buffer.clone()
    }

    fn get_indices(&self) -> Vec<u32> {
        self.index_buffer.clone()
    }

    fn set_color(&mut self, color: [f32; 4]) {
        // Only change the color of the square
        for vertex in &mut self.vertex_buffer[0..4] {
            vertex.color = color;
        }
    }

    fn set_model_matrix(&mut self, matrix: Matrix) {
        for vertex in &mut self.vertex_buffer {
            vertex.model = matrix.get_matrix();
        }
    }

    fn get_num_vertices(&self) -> usize {
        self.vertex_buffer.len()
    }
    
    fn get_num_indices(&self) -> usize {
        self.index_buffer.len()
    }

    fn get_id(&self) -> u32 {
        match self.vertex_buffer.get(0) {
            Some(vertex) => vertex.id,
            None => 0,
        }
    }

    fn set_id(&mut self, id: u32) {
        // Only change the id of the square
        for vertex in &mut self.vertex_buffer[0..4] {
            vertex.id = id;
        }
    }

    fn get_tex_id(&self) -> u32 {
        match self.vertex_buffer.get(0) {
            Some(vertex) => vertex.tex_id,
            None => 0,
        }
    }

    fn set_tex_id(&mut self, id: u32) {
        // only change the tex_id of the square not its outline
        for vertex in &mut self.vertex_buffer[0..4] {
            vertex.tex_id = id;
        }
    }

    fn set_rotation(&mut self, rotation: f32) {
        for vertex in &mut self.vertex_buffer {
            vertex.rotation = Matrix4::from_angle_z(cgmath::Rad(rotation)).into()
        }
    }

    fn clone_primitive(&self) -> Box<dyn PrimitiveShape> {
        Box::new(self.clone())
    }
}

pub mod primitive;
pub mod shape_path;

use shape_path::ShapePath;
use crate::render::matrix::Matrix;
use crate::parse::{Yaml, SavableBlueprint};

use cgmath::Matrix4;
use cgmath::Rad;
use lyon::math::{point, Point};
use lyon::path::Path;
use lyon::tessellation::*;

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
            model: Matrix::new().get_matrix(),
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

pub trait Shape: Sync + Send{
    fn get_vertices(&self) -> Vec<Vertex>;
    fn get_indices(&self) -> Vec<u32>;
    fn set_color(&mut self, color: [f32; 4]);
    fn set_model_matrix(&mut self, matrix: Matrix);
    fn get_num_vertices(&self) -> usize;
    fn get_num_indices(&self) -> usize;
    fn get_id(&self) -> u32;
    fn set_id(&mut self, id: u32);
    fn get_tex_id(&self) -> u32;
    fn set_tex_id(&mut self, id: u32);
    fn set_rotation(&mut self, rotation: f32);
    fn was_clicked(&self, id: u32) -> bool {
        self.get_id() == id
    }
    fn clone_shape(&self) -> Box<dyn Shape>;
}

// Path Shape will create a filled shape from the given path

#[derive(Clone)]
pub struct PathShape {
    path: ShapePath,
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<u32>,
    should_outline: bool,
    outline: StrokeShape,
}

impl PathShape {
    pub fn new(path: ShapePath, id: u32) -> Self {
        let mut geometry: VertexBuffers<Vertex, u32> = VertexBuffers::new();

        let mut tessellator = FillTessellator::new();

        {
            tessellator.tessellate(
                &path.build_path(), 
                &FillOptions::default(), 
                &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
                    Vertex {
                        position: vertex.position().to_array(),
                        id,
                        .. Default::default()
                    }
                }),
            ).unwrap();
        }

        let vertex_buffer = geometry.vertices.to_vec();

        let index_buffer = geometry.indices.to_vec();

        let outline = StrokeShape::new(&path.build_path(), 0, &StrokeOptions::default().with_line_width(crate::quilt::block::Block::SHAPE_BORDER_WIDTH));

        Self {
            path,
            vertex_buffer,
            index_buffer,
            should_outline: true,
            outline,
        }
    }

    pub fn new_with_line_width(path: ShapePath, id: u32, line_width: f32) -> Self {
        let mut geometry: VertexBuffers<Vertex, u32> = VertexBuffers::new();

        let mut tessellator = FillTessellator::new();

        {
            tessellator.tessellate(
                &path.build_path(), 
                &FillOptions::default(), 
                &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
                    Vertex {
                        position: vertex.position().to_array(),
                        id,
                        .. Default::default()
                    }
                }),
            ).unwrap();
        }

        let vertex_buffer = geometry.vertices.to_vec();

        let index_buffer = geometry.indices.to_vec();

        let outline = StrokeShape::new(&path.build_path(), 0, &StrokeOptions::default().with_line_width(line_width));

        Self {
            path,
            vertex_buffer,
            index_buffer,
            should_outline: true,
            outline,
        }
    }

    pub fn circle(center: lyon::math::Point, radius: f32, start_angle_radians: f32, end_angle_radians: f32, id: u32) -> Self {
        let mut path = ShapePath::new();

        path.arc_to(center, radius, start_angle_radians, end_angle_radians);
        path.line_to(center);
        path.close();

        Self::new(path, id)
    }

    pub fn triangle(pos1: (f32, f32), pos2: (f32, f32), pos3: (f32, f32), id: u32) -> Self {
        let mut shape_path = ShapePath::new();

        shape_path.move_to(pos1.into());
        shape_path.line_to(pos2.into());
        shape_path.line_to(pos3.into());
        shape_path.line_to(pos1.into());

        Self::new(shape_path, id)
    }

    pub fn square(x: f32, y: f32, width: f32, height: f32, id: u32) -> Self {
        let mut shape_path = ShapePath::new();

        shape_path.move_to(point(x, y));
        shape_path.line_to(point(x + width, y));
        shape_path.line_to(point(x + width, y + height));
        shape_path.line_to(point(x, y + height));
        shape_path.line_to(point(x, y));

        Self::new(shape_path, id)
    }

    pub fn square_with_line_width(x: f32, y: f32, width: f32, height: f32, id: u32, line_width: f32) -> Self {
        let mut shape_path = ShapePath::new();

        shape_path.move_to(point(x, y));
        shape_path.line_to(point(x + width, y));
        shape_path.line_to(point(x + width, y + height));
        shape_path.line_to(point(x, y + height));
        shape_path.line_to(point(x, y));

        Self::new_with_line_width(shape_path, id, line_width)
    }
}

impl Shape for PathShape {
    fn get_vertices(&self) -> Vec<Vertex> {
        let mut vb = self.vertex_buffer.clone();
        
        if self.should_outline {
            vb.append(&mut self.outline.get_vertices());
        }
        
        vb
    }
    
    fn get_indices(&self) -> Vec<u32> {
        let mut ib = self.index_buffer.clone();
        
        if self.should_outline {
            ib.reserve(self.outline.get_num_indices());
    
            for index in self.outline.get_indices() {
                ib.push(index + self.vertex_buffer.len() as u32);
            }
        }

        ib
    }

    fn set_color(&mut self, color: [f32; 4]) {
        for vertex in &mut self.vertex_buffer {
            vertex.color = color;
        }
    }

    fn set_model_matrix(&mut self, matrix: Matrix) {
        for vertex in &mut self.vertex_buffer {
            vertex.model = matrix.get_matrix();
        }

        self.outline.set_model_matrix(matrix);
    }

    fn get_num_vertices(&self) -> usize {
        self.vertex_buffer.len() + self.outline.get_num_vertices()
    }

    fn get_num_indices(&self) -> usize {
        self.index_buffer.len() + self.outline.get_num_indices()
    }

    fn get_id(&self) -> u32 {
        match self.vertex_buffer.get(0) {
            Some(vertex) => vertex.id,
            None => 0,
        }
    }

    fn set_id(&mut self, id: u32) {
        for vertex in &mut self.vertex_buffer {
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
        for vertex in &mut self.vertex_buffer {
            vertex.tex_id = id;
        }
    }

    fn set_rotation(&mut self, rotation: f32) {
        for vertex in &mut self.vertex_buffer {
            vertex.rotation = Matrix4::from_angle_z(cgmath::Rad(rotation)).into()
        }

        self.outline.set_rotation(rotation);
    }

    fn clone_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}

impl SavableBlueprint for PathShape {
    fn from_save_blueprint(yaml: Yaml) -> Box<Self> where Self: Sized {
        let path = ShapePath::from_save_blueprint(yaml);

        Box::new(Self::new(*path, 0))
    }
}

// Stroke Shape will create a border for the given path

#[derive(Clone)]
pub struct StrokeShape {
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<u32>,
}

impl StrokeShape {

    pub fn new(path: &Path, id: u32, stroke_options: &StrokeOptions) -> Self {
        let stroke_options = stroke_options.clone().with_tolerance(0.001);


        let mut buffers: VertexBuffers<Point, u16> = VertexBuffers::new();

        {
            let mut vertex_builder = lyon::tessellation::geometry_builder::simple_builder(&mut buffers);
            let mut tessellator = StrokeTessellator::new();

            tessellator.tessellate(path, &stroke_options, &mut vertex_builder).expect("error making stroke");
        }

        let i = buffers.indices;
        let mut index_buffer = Vec::with_capacity(i.len());

        for index in i {
            index_buffer.push(index as u32);
        }

        

        let v = buffers.vertices;
        let mut vertex_buffer = Vec::with_capacity(v.len());

        for vertex in v {
            vertex_buffer.push(Vertex {
                position: [vertex.x, vertex.y],
                color: [0.0, 0.0, 0.0, 1.0],
                id,
                model: crate::render::matrix::Matrix::new().get_matrix(),
                .. Default::default()
                // tex_id: 1,
            })
        }

        Self {
            vertex_buffer,
            index_buffer,
        }
    }


    pub fn square(x: f32, y: f32, width: f32, height: f32, id: u32, stroke_options: &StrokeOptions) -> Self {
        let mut path = Path::svg_builder();
        path.move_to(point(x, y));
        path.line_to(point(x + width, y));
        path.line_to(point(x + width, y + height));
        path.line_to(point(x, y + height));
        path.line_to(point(x, y));
        path.close();
        let path = path.build();

        Self::new(&path, id, stroke_options)
    }
}

impl Shape for StrokeShape {
    fn get_vertices(&self) -> Vec<Vertex> {
        self.vertex_buffer.clone()
    }

    fn get_indices(&self) -> Vec<u32> {
        self.index_buffer.clone()
    }

    fn set_color(&mut self, color: [f32; 4]) {
        for vertex in &mut self.vertex_buffer {
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
        for vertex in &mut self.vertex_buffer {
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
        for vertex in &mut self.vertex_buffer {
            vertex.tex_id = id;
        }
    }

    fn set_rotation(&mut self, rotation: f32) {
        for vertex in &mut self.vertex_buffer {
            vertex.rotation = Matrix4::from_angle_z(cgmath::Rad(rotation)).into()
        }
    }

    fn clone_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}

pub fn draw<'a, U: glium::uniforms::Uniforms>(shape: &(&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>), frame: &mut impl glium::Surface, program: &glium::Program, uniforms: &U, draw_parameters: &glium::DrawParameters<'_>) {
    frame.draw(shape.0, shape.1, program, uniforms, draw_parameters).unwrap();
}
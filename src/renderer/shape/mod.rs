pub mod primitive;
pub mod shape_path;

use shape_path::ShapePath;
use crate::renderer::matrix::Matrix;
use crate::parse::{Yaml, SavableBlueprint, Savable, LinkedHashMap, SaveData};
use crate::program::quilt::block::Block;
use crate::renderer::vertex::Vertex;
use crate::renderer::textures;

use cgmath::Matrix4;
use lyon::math::{point, Point};
use lyon::tessellation::*;
use std::io::Write;

pub trait Shape: Sync + Send + SavableBlueprint + Savable + PrimitiveShape {
    fn clone_shape(&self) -> Box<dyn Shape>;
}

pub trait PrimitiveShape: Sync + Send {
    fn get_vertices(&self) -> Vec<Vertex>;
    fn get_indices(&self) -> Vec<u32>;
    fn set_color(&mut self, color: [f32; 4]);
    fn set_model_matrix(&mut self, matrix: Matrix);
    fn get_model_matrix(&self) -> Matrix;
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
    fn clone_primitive(&self) -> Box<dyn PrimitiveShape>;
}

// Path Shape will create a filled shape from the given path

#[derive(Clone)]
pub struct PathShape {
    path: ShapePath,
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<u32>,
    should_outline: bool,
    outline: StrokeShape,
    rotation: f32,
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

        let outline = StrokeShape::new(path.clone(), 0, &StrokeOptions::default().with_line_width(Block::SHAPE_BORDER_WIDTH));

        Self {
            path,
            vertex_buffer,
            index_buffer,
            should_outline: true,
            outline,
            rotation: 0.0,
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

        let outline = StrokeShape::new(path.clone(), 0, &StrokeOptions::default().with_line_width(line_width));

        Self {
            path,
            vertex_buffer,
            index_buffer,
            should_outline: true,
            outline,
            rotation: 0.0
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
        shape_path.close();

        Self::new(shape_path, id)
    }

    pub fn square(x: f32, y: f32, width: f32, height: f32, id: u32) -> Self {
        let mut shape_path = ShapePath::new();

        shape_path.move_to(point(x, y));
        shape_path.line_to(point(x + width, y));
        shape_path.line_to(point(x + width, y + height));
        shape_path.line_to(point(x, y + height));
        shape_path.line_to(point(x, y));
        shape_path.close();

        Self::new(shape_path, id)
    }

    pub fn square_with_line_width(x: f32, y: f32, width: f32, height: f32, id: u32, line_width: f32) -> Self {
        let mut shape_path = ShapePath::new();

        shape_path.move_to(point(x, y));
        shape_path.line_to(point(x + width, y));
        shape_path.line_to(point(x + width, y + height));
        shape_path.line_to(point(x, y + height));
        shape_path.line_to(point(x, y));
        shape_path.close();

        Self::new_with_line_width(shape_path, id, line_width)
    }
}

impl SavableBlueprint for PathShape {
    fn to_save_blueprint(&self) -> Yaml {
        self.path.to_save_blueprint()
    }
    fn from_save_blueprint(yaml: Yaml) -> Box<Self> where Self: Sized {
        let path = ShapePath::from_save_blueprint(yaml);

        Box::new(Self::new(*path, 0))
    }
}

impl Savable for PathShape {
    // path: ShapePath,
    // vertex_buffer: Vec<Vertex>,
    // index_buffer: Vec<u32>,
    // should_outline: bool,
    // outline: StrokeShape,
    // rotation: f32,

    fn to_save(&self, save_data: &mut SaveData) -> Yaml {

        if self.get_tex_id() > 0 {

            let texture = &textures::get_textures().get((self.get_tex_id() - 1) as usize).unwrap();

            let file_name = format!{"{}.png", texture.get_hash()};
            
            if !save_data.files_written.contains(&file_name) {
                let mut buffer = Vec::new();
                texture.write_to(&mut buffer, image::ImageOutputFormat::Png).unwrap();
                save_data.files_written.push(file_name.clone());
                let writer = save_data.writer.as_mut().unwrap();
                
                let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
                writer.start_file(file_name.clone(), options).unwrap();
        
                writer.write(&buffer).unwrap();
            }

            return LinkedHashMap::create(vec![
                ("path", self.path.to_save_blueprint()),
                ("color", (&self.vertex_buffer[0].color).into()),
                ("texture", file_name.as_str().into())
            ])
        } 

        LinkedHashMap::create(vec![
            ("path", self.path.to_save_blueprint()),
            ("color", (&self.vertex_buffer[0].color).into()),
            ("texture", "".into()),
        ])

    }

    fn from_save(yaml: Yaml, _save_data: &mut SaveData) -> Box<Self> where Self: Sized {
        let map = LinkedHashMap::from(yaml);

        let path = ShapePath::from_save_blueprint(map.get("path").clone());

        // Load texture
        let texture_path = String::from(map.get("texture"));
        let texture = if let Some(location) = texture_path.find('.') {
            let texture_hash = &texture_path[0..location];
    
            textures::get_texture_by_hash(texture_hash)
        } else {None};

        let mut s = Self::new(*path, 0);
        
        // set_color will set the tex id to 0, so do it before setting texture_id
        s.set_color(map.get("color").into());

        if let Some(texture) = texture {
            s.set_tex_id(texture.get_texture_index() as u32 + 1);
        }

        Box::new(s)
    }
}

impl PrimitiveShape for PathShape {
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
            vertex.tex_id = 0;
        }
    }

    fn set_model_matrix(&mut self, matrix: Matrix) {
        for vertex in &mut self.vertex_buffer {
            vertex.model = matrix.get_matrix();
        }

        self.outline.set_model_matrix(matrix);
    }

    fn get_model_matrix(&self) -> Matrix {
        Matrix::new_with_data(self.vertex_buffer[0].model)
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
        self.rotation = rotation;

        for vertex in &mut self.vertex_buffer {
            vertex.rotation = Matrix4::from_angle_z(cgmath::Rad(rotation)).into()
        }

        self.outline.set_rotation(rotation);
    }

    fn clone_primitive(&self) -> Box<dyn PrimitiveShape> {
        Box::new(self.clone())
    }
}

impl Shape for PathShape{
    fn clone_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}

// Stroke Shape will create a border for the given path

#[derive(Clone)]
pub struct StrokeShape {
    path: ShapePath,
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<u32>,
}

impl StrokeShape {

    pub fn new(path: ShapePath, id: u32, stroke_options: &StrokeOptions) -> Self {
        let stroke_options = stroke_options.clone().with_tolerance(0.001);


        let mut buffers: VertexBuffers<Point, u16> = VertexBuffers::new();

        {
            let mut vertex_builder = lyon::tessellation::geometry_builder::simple_builder(&mut buffers);
            let mut tessellator = StrokeTessellator::new();

            tessellator.tessellate(&path.build_path(), &stroke_options, &mut vertex_builder).expect("error making stroke");
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
                model: crate::renderer::matrix::Matrix::new().get_matrix(),
                .. Default::default()
                // tex_id: 1,
            })
        }

        Self {
            path,
            vertex_buffer,
            index_buffer,
        }
    }


    pub fn square(x: f32, y: f32, width: f32, height: f32, id: u32, stroke_options: &StrokeOptions) -> Self {
        let mut path = ShapePath::new();

        path.move_to(point(x, y));
        path.line_to(point(x + width, y));
        path.line_to(point(x + width, y + height));
        path.line_to(point(x, y + height));
        path.line_to(point(x, y));
        path.close();

        Self::new(path, id, stroke_options)
    }
}

impl SavableBlueprint for StrokeShape {
    fn to_save_blueprint(&self) -> Yaml {
        self.path.to_save_blueprint()
    }
    fn from_save_blueprint(yaml: Yaml) -> Box<Self> where Self: Sized {
        let path = ShapePath::from_save_blueprint(yaml);

        Box::new(Self::new(*path, 0, &StrokeOptions::default().with_line_width(Block::SHAPE_BORDER_WIDTH)))
    }
}

impl Savable for StrokeShape {
    fn to_save(&self, _save_data: &mut SaveData) -> Yaml {
        // NOTE doesn't actually work
        self.path.to_save_blueprint()
    }
    fn from_save(yaml: Yaml, _save_data: &mut SaveData) -> Box<Self> where Self: Sized {
        // NOTE doesn't actually work
        let path = ShapePath::from_save_blueprint(yaml);

        Box::new(Self::new(*path, 0, &StrokeOptions::default().with_line_width(Block::SHAPE_BORDER_WIDTH)))
    }
}

impl Shape for StrokeShape{
    fn clone_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}

impl PrimitiveShape for StrokeShape {
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

    fn get_model_matrix(&self) -> Matrix {
        Matrix::new_with_data(self.vertex_buffer[0].model)
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

    fn clone_primitive(&self) -> Box<dyn PrimitiveShape> {
        Box::new(self.clone())
    }
}

pub fn draw<'a, U: glium::uniforms::Uniforms>(shape: &(&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>), frame: &mut impl glium::Surface, program: &glium::Program, uniforms: &U, draw_parameters: &glium::DrawParameters<'_>) {
    frame.draw(shape.0, shape.1, program, uniforms, draw_parameters).unwrap();
}
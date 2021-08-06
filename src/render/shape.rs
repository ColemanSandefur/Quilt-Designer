use crate::render::matrix::Matrix;

use lyon::math::{point, Point};
use lyon::path::Path;
use lyon::tessellation::*;
use lyon::path::{ArcFlags};
use lyon::path::builder::SvgPathBuilder;
use lyon::geom::vector;
use lyon::geom::Angle;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub model: [[f32;4]; 4],
    pub id: u32,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: [0.0; 2],
            color: [1.0; 4],
            model: Matrix::new().get_matrix(),
            id: 0,
        }
    }
}

impl Vertex {
    pub fn to_point(&self) -> Point {
        point(self.position[0], self.position[1])
    }
}

implement_vertex!(Vertex, position, color, model, id);

pub trait Shape: Sync + Send{
    fn get_vertices(&self) -> Vec<Vertex>;
    fn get_indices(&self) -> Vec<u32>;
    fn set_color(&mut self, color: [f32; 4]);
    fn set_model_matrix(&mut self, matrix: Matrix);
    fn get_num_vertices(&self) -> usize;
    fn get_num_indices(&self) -> usize;
    fn get_id(&self) -> u32;
    fn set_id(&mut self, id: u32);
    fn was_clicked(&self, id: u32) -> bool {
        self.get_id() == id
    }
    fn clone_shape(&self) -> Box<dyn Shape>;
}

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

        let mut outline = Path::svg_builder();
        outline.move_to(point(pos1.0, pos1.1));
        outline.line_to(point(pos2.0, pos2.1));
        outline.line_to(point(pos3.0, pos3.1));
        outline.line_to(point(pos1.0, pos1.1));
        outline.close();
        let outline = outline.build();

        let stroke = StrokeShape::new(&outline, 0, &StrokeOptions::default().with_line_width(crate::quilt::block::Block::SHAPE_BORDER_WIDTH));

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

impl Shape for Triangle {
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

    fn clone_shape(&self) -> Box<dyn Shape> {
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

impl Shape for Square {
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

    fn clone_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}

// Path Shape will create a filled shape from the given path

#[derive(Clone)]
pub struct PathShape {
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<u32>,
    should_outline: bool,
    outline: StrokeShape,
}

impl PathShape {
    pub fn new(path: &Path, id: u32) -> Self {
        let mut geometry: VertexBuffers<Vertex, u32> = VertexBuffers::new();

        let mut tessellator = FillTessellator::new();

        {
            tessellator.tessellate(
                path, 
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

        let outline = StrokeShape::new(&path, 0, &StrokeOptions::default().with_line_width(crate::quilt::block::Block::SHAPE_BORDER_WIDTH));

        Self {
            vertex_buffer,
            index_buffer,
            should_outline: true,
            outline,
        }
    }

    pub fn relative_arc_to(start: lyon::math::Point, radius: f32, destination: lyon::math::Vector, draw_clockwise: bool, large_arc: bool, id: u32) -> Self {

        let mut path = Path::svg_builder().flattened(0.001);
        path.move_to(start);
        path.relative_arc_to(
            vector(radius, radius), 
            Angle {radians: 0.0}, 
            ArcFlags {
                sweep: !draw_clockwise,
                large_arc: large_arc
            }, 
            destination
        );
        path.close();
        let path = path.build();

        Self::new(&path, id)
    }

    pub fn circle(center: lyon::math::Point, radius: f32, start_angle_radians: f32, end_angle_radians: f32, id: u32) -> Self {
        let total_angle = end_angle_radians - start_angle_radians;

        // You need to draw 2 arcs if you are doing a complete circle
        if total_angle == 2.0 * std::f32::consts::PI {
            let mut path = Path::svg_builder().flattened(0.001);
            path.move_to(point(radius + center.x, center.y));
            path.arc_to(vector(radius, radius), Angle {radians: 0.0}, ArcFlags::default(), point(-radius + center.x, center.y));
            path.arc_to(vector(radius, radius), Angle {radians: 0.0}, ArcFlags::default(), point( radius + center.x, center.y));
            let path = path.build();

            return Self::new(&path, id);
        }

        let mut arc_flags = ArcFlags {
            sweep: true, // which way to draw (false => Clockwise, true => Counter Clockwise)
            large_arc: false,
        };

        // determines which direction the arc starts drawing, should draw clockwise when the total angle is negative
        // sweep goes clockwise when false
        if total_angle < 0.0 {
            arc_flags.sweep = false;
        }

        if total_angle.abs() > std::f32::consts::PI {
            arc_flags.large_arc = true;
        }

        let start_x = radius * start_angle_radians.cos() + center.x;
        let start_y = radius * start_angle_radians.sin() + center.y;

        let stop_x = radius * end_angle_radians.cos() + center.x;
        let stop_y = radius * end_angle_radians.sin() + center.y;

        let mut path = Path::svg_builder().flattened(0.001);
        path.move_to(point(start_x, start_y));
        path.arc_to(vector(radius, radius), Angle {radians: 0.0}, arc_flags, point(stop_x, stop_y));
        path.line_to(center);
        path.close();
        let path = path.build();

        Self::new(&path, id)
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

    fn clone_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
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
                model: crate::render::matrix::Matrix::new().get_matrix()
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

    fn clone_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}

pub fn draw<'a, U: glium::uniforms::Uniforms>(shape: &(&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>), frame: &mut impl glium::Surface, program: &glium::Program, uniforms: &U, draw_parameters: &glium::DrawParameters<'_>) {
    frame.draw(shape.0, shape.1, program, uniforms, draw_parameters).unwrap();
}
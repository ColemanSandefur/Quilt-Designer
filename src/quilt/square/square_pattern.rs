use crate::render::object::{ShapeDataStruct};
use lyon::path::*;
use lyon::math::point;

#[derive(Clone)]
pub struct SquarePattern {
    shapes: Vec<Box<ShapeDataStruct>>,
}

impl SquarePattern {
    pub fn new(mut shapes: Vec<Box<ShapeDataStruct>>) -> Self {

        // add black outline to square pattern
        let mut path = Path::svg_builder();
        path.move_to(point(0.0, 0.0));
        path.line_to(point(0.0, 1.0));
        path.line_to(point(1.0, 1.0));
        path.line_to(point(1.0, 0.0));
        path.close();
        let path = path.build();

        shapes.push(
            Box::new(ShapeDataStruct::new(
                Box::new(crate::render::shape::StrokeShape::new(&path, 0, &lyon::lyon_tessellation::StrokeOptions::default().with_line_width(0.05)),
            )),
        ));

        Self {
            shapes,
        }
    }
    pub fn get_shapes(&self) -> &Vec<Box<ShapeDataStruct>> {
        &self.shapes
    }

    pub fn draw(&mut self, surface: &mut impl glium::Surface, facade: & impl glium::backend::Facade) {

        // get num elements to avoid resizing vector
        let mut total_vertices = 0;
        let mut total_indices = 0;

        for shape in &self.shapes {
            total_vertices = shape.shape.get_num_vertices();
            total_indices = shape.shape.get_num_indices();
        }

        let mut vb_vec: Vec<crate::render::shape::Vertex> = Vec::with_capacity(total_vertices);
        let mut ib_vec = Vec::with_capacity(total_indices);

        for shape in &mut self.shapes {
            // add to ib_vec
            let indices = shape.shape.get_indices();
            let start_index = vb_vec.len();
    
            for i in 0..indices.len() {
                ib_vec.push(start_index as u32 + indices[i]);
            }
            
            // add to vb_vec
            for vert in &mut shape.shape.get_vertices() {
                let mut vert = vert.clone();
                vert.position[0] = vert.position[0] *  2.0 - 1.0;
                vert.position[1] = vert.position[1] * -2.0 + 1.0;

                vb_vec.push(vert);
            }

        }

        let vb = glium::VertexBuffer::new(facade, &vb_vec).expect("Unable to initialize vb for square pattern");
        let ib = glium::IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, &ib_vec).expect("Unable to initialize ib for square pattern");

        let material = crate::render::material::material_manager::get_material_manager().get_solid_color_material();

        let world_transform = crate::render::matrix::WorldTransform {
            projection: crate::render::matrix::Matrix::new(),
            world: crate::render::matrix::Matrix::new(),
        };

        material.draw(&(&vb, &ib), surface, &world_transform, &crate::render::matrix::Matrix::new(), &Default::default());
    }
}

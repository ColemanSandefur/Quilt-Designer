use crate::render::object::{ShapeDataStruct};

#[derive(Clone)]
pub struct SquarePattern {
    shapes: Vec<Box<ShapeDataStruct>>,
}

impl SquarePattern {
    pub fn new(mut shapes: Vec<Box<ShapeDataStruct>>) -> Self {

        if let Some(shape) = shapes.get_mut(1) {
            shape.shape.set_color([0.2, 0.2, 0.2, 1.0]);
        }

        Self {
            shapes,
        }
    }
    pub fn get_shapes(&self) -> &Vec<Box<ShapeDataStruct>> {
        &self.shapes
    }

    pub fn draw(&mut self, surface: &mut impl glium::Surface, facade: & impl glium::backend::Facade, materail_manager: &crate::render::material::material_manager::MaterialManager) {

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
            // add to vb_vec
            for vert in &mut shape.shape.get_vertices() {
                let mut vert = vert.clone();
                vert.position[0] = vert.position[0] *  2.0 - 1.0;
                vert.position[1] = vert.position[1] * -2.0 + 1.0;

                vb_vec.push(vert);
            }

            // add to ib_vec
            let indices = shape.shape.get_indices();
            let start_index = ib_vec.len();
    
            for i in 0..indices.len() {
                ib_vec.push(start_index as u32 + indices[i]);
            }
        }

        let vb = glium::VertexBuffer::new(facade, &vb_vec).expect("Unable to initialize vb for square pattern");
        let ib = glium::IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, &ib_vec).expect("Unable to initialize ib for square pattern");

        let material = materail_manager.get_solid_color_material();

        let world_transform = crate::render::matrix::WorldTransform {
            projection: crate::render::matrix::Matrix::new(),
            world: crate::render::matrix::Matrix::new(),
        };

        material.draw(&(&vb, &ib), surface, &world_transform, &crate::render::matrix::Matrix::new(), &Default::default());
    }
}

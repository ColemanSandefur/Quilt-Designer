
use crate::render::material::{*, material_manager::{MaterialManager, MaterialType}};
use crate::render::matrix::{Matrix, WorldTransform};
use crate::render::shape::Shape;

use lyon::path::{ArcFlags, Path};
use lyon::path::builder::SvgPathBuilder;
use lyon::math::point;
use lyon::geom::vector;
use lyon::geom::Angle;

// Everything rendered will be a Shape Object, this will be added to the renderer's list
// the renderer will then handle the drawing of the object

pub struct ShapeDataStruct {
    pub shape: Box<dyn Shape>,
    pub shader: Box<dyn Material>,
    pub click_shader: ClickMaterial,
}

impl ShapeDataStruct {
    pub fn new(shape: Box<dyn Shape>, shader: Box<dyn Material>, click_shader: ClickMaterial) -> Self {
        Self {
            shape,
            shader,
            click_shader,
        }
    }
}

pub trait ShapeObject {
    fn get_shapes(&self) -> &Vec<Box<ShapeDataStruct>>;
    fn get_shapes_mut(&mut self) -> &mut Vec<Box<ShapeDataStruct>>;
    fn get_model_transform(&self) -> &Matrix;
    fn get_model_transform_mut(&mut self) -> &mut Matrix;
    fn draw(&mut self, frame: &mut glium::Frame, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>);
    fn draw_click(&mut self, frame: &mut glium::Frame, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>);
}

pub struct DefaultShapeObject {
    shapes: Vec<Box<ShapeDataStruct>>,
    model_transform: Matrix,
}

impl DefaultShapeObject {
    pub fn new(display: &dyn glium::backend::Facade, shaders: &mut MaterialManager) -> Self {

        let mut half_circle = Path::svg_builder().flattened(0.0001);
        half_circle.move_to(point(0.0, -0.25));
        half_circle.relative_arc_to(
            vector(0.25, 0.25),
            Angle {radians: 3.14},
            ArcFlags {
                large_arc: true,
                sweep: true
            },
            vector(0.0, 0.5),
        );
        half_circle.close();
        let half_circle = half_circle.build();

        let shapes: Vec<Box<ShapeDataStruct>> = vec!{
            Box::new(ShapeDataStruct::new(
                Box::new(crate::render::shape::Square::with_width_height(display, -0.25, -0.25, 0.5, 0.5)), 
                shaders.get_material(MaterialType::SolidColorMaterial).unwrap(),
                shaders.get_click_material()
            )),
            Box::new(ShapeDataStruct::new(
                Box::new(crate::render::shape::PathShape::from_vertices(display, &vec!{
                    crate::render::shape::Vertex {position: [0.0, -0.5]},
                    crate::render::shape::Vertex {position: [-0.5, 0.2]},
                    crate::render::shape::Vertex {position: [0.0, 0.0]},
                    crate::render::shape::Vertex {position: [0.5, 0.5]},
                })),
                {
                    Box::new(shaders.get_material(MaterialType::SolidColorMaterial).unwrap().as_any().downcast_ref::<SolidColorMaterial>().unwrap().create_from_existing([0.2, 0.2, 1.0, 1.0]))
                },
                shaders.get_click_material(),
            )),
            Box::new(ShapeDataStruct::new(
                Box::new(
                    crate::render::shape::PathShape::new(display, half_circle),
                ),
                {
                    Box::new(shaders.get_material(MaterialType::SolidColorMaterial).unwrap().as_any().downcast_ref::<SolidColorMaterial>().unwrap().create_from_existing([0.2, 1.0, 1.0, 1.0]))
                },
                shaders.get_click_material(),
            ))
        };

        let model_transform = Matrix::new();

        Self {
            shapes,
            model_transform,
        }
    }

    
}

impl ShapeObject for DefaultShapeObject {
    fn get_shapes(&self) -> &Vec<Box<ShapeDataStruct>> {
        &self.shapes
    }
    fn get_shapes_mut(&mut self) -> &mut Vec<Box<ShapeDataStruct>> {
        &mut self.shapes
    }
    fn get_model_transform(&self) -> &Matrix {
        &self.model_transform
    }
    fn get_model_transform_mut(&mut self) -> &mut Matrix {
        &mut self.model_transform
    }

    fn draw(&mut self, frame: &mut glium::Frame, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>) {
        for shape_data in &self.shapes {
            // crate::shapes::draw(&shape_data.shape, frame, program, &uniforms.add("local", shape_data.local_transform).add("color", shape_data.color), draw_parameters);
            shape_data.shader.draw(&shape_data.shape, frame, world_transform, &self.model_transform, draw_parameters);
        }
    }

    fn draw_click(&mut self, frame: &mut glium::Frame, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>) {
        for shape_data in &self.shapes {
            shape_data.click_shader.draw(&shape_data.shape, frame, world_transform, &self.model_transform, draw_parameters);
        }
    }
}
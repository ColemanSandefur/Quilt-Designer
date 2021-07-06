use crate::render::material::material_manager::MaterialManager;
use crate::render::matrix::{Matrix, WorldTransform};
use crate::render::object::{ShapeObject, DefaultShapeObject};
use glium::Surface;

#[allow(dead_code)]
pub struct Renderer {
    shaders: MaterialManager,
    objects: Vec<Box<dyn ShapeObject>>,
    world_transform: Matrix,
}

impl Renderer {
    pub fn new(display: &dyn glium::backend::Facade) -> Self {
        let shaders = MaterialManager::load_all(display);

        let mut objects: Vec<Box<dyn ShapeObject>> = vec!{
            Box::new(DefaultShapeObject::new(display, &shaders)),
            Box::new(DefaultShapeObject::new(display, &shaders)),
        };

        objects[1].get_model_transform_mut().translate(-1.0, 0.5, 0.0);
        objects[1].get_model_transform_mut().set_scale(0.5, 0.5, 1.0);

        let world_transform = Matrix::new_with_data([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0, 1.0],
        ]);

        Self {
            shaders,
            objects,
            world_transform,
        }
    }

    pub fn draw(&mut self, target: &mut glium::Frame) {
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let projection = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            Matrix::new_with_data([
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ])
        };

        let global_transform = WorldTransform {
            projection: projection,
            world: self.world_transform,
        };

        self.objects[0].get_model_transform_mut().translate(0.0, 0.0, 0.01);

        for shape in &mut self.objects {
            shape.draw(target, &global_transform, &Default::default());
        }
    }
}
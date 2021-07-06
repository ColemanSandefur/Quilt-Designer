use crate::render::material::*;

use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum MaterialType {
    SolidColorMaterial
}

pub struct MaterialManager {
    materials: HashMap<MaterialType, Box<dyn Material>>,
}

impl MaterialManager {
    pub fn get_material(&self, m: MaterialType) -> Option<Box<dyn Material>> {
        match self.materials.get(&m) {
            Some(v) => {
                Some((*v).clone())
            },
            None => None
        }
    }

    pub fn load_all(shader: &dyn glium::backend::Facade) -> Self {
        let mut materials: HashMap<MaterialType, Box<dyn Material>> = HashMap::new();

        materials.insert(
            MaterialType::SolidColorMaterial,
            Box::new(SolidColorMaterial::new(Self::load_from_file(std::path::Path::new("./shaders/solid_color"), shader), [1.0, 1.0, 1.0, 1.0]))
        );

        Self {
            materials
        }
    }

    fn load_from_file(path: &std::path::Path, display: &dyn glium::backend::Facade) -> Rc<glium::Program>{
        use std::io::Read;

        let mut fragment_file = std::fs::File::open(path.join("fragment.txt")).expect(
            format!("unable to find file {}", std::env::current_dir().unwrap().join(path).join("fragment.txt").into_os_string().to_str().unwrap()).as_str()
        );
        
        let mut vertex_file = std::fs::File::open(path.join("vertex.txt")).expect(
            format!("unable to find file {}", std::env::current_dir().unwrap().join(path).join("vertex.txt").into_os_string().to_str().unwrap()).as_str()
        );

        let mut fragment_shader_src = String::new();
        fragment_file.read_to_string(&mut fragment_shader_src).unwrap();

        let mut vertex_shader_src = String::new();
        vertex_file.read_to_string(&mut vertex_shader_src).unwrap();

        Rc::new(glium::Program::from_source(display, vertex_shader_src.as_str(), fragment_shader_src.as_str(), None).unwrap())
    }
}
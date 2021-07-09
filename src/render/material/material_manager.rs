use crate::render::material::*;

use std::collections::HashMap;
use std::rc::Rc;
use rand::Rng;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum MaterialType {
    SolidColorMaterial,
    ClickMaterial,
}

pub struct MaterialManager {
    materials: HashMap<MaterialType, Box<dyn Material>>,
    click_material: ClickMaterial,
    click_vec: Vec<[f32; 4]>,
}

impl MaterialManager {
    pub fn get_click_material(&mut self) -> ClickMaterial {
        let mut rng = rand::thread_rng();

        let mut color = [
            rng.gen(),
            rng.gen(),
            rng.gen(),
            1.0
        ];

        while self.click_vec.contains(&color) {
            color = [
                rng.gen(),
                rng.gen(),
                rng.gen(),
                1.0
            ];
        }

        self.click_vec.push(color.clone());

        let mut material = self.click_material.clone();

        material.color = color.clone();

        material
    }

    pub fn get_material(&self, m: MaterialType) -> Option<Box<dyn Material>> {
        

        match self.materials.get(&m) {
            Some(v) => {
                Some((*v).clone_material())
            },
            None => None
        }
    }

    pub fn load_all(display: &dyn glium::backend::Facade) -> Self {
        let mut materials: HashMap<MaterialType, Box<dyn Material>> = HashMap::new();

        materials.insert(
            MaterialType::SolidColorMaterial,
            Box::new(SolidColorMaterial::new(Self::load_from_file(std::path::Path::new("./shaders/solid_color"), display)))
        );

        let click_material = ClickMaterial::new(Self::load_from_file(std::path::Path::new("./shaders/solid_color"), display), [1.0, 1.0, 1.0, 1.0]);

        let click_vec = Vec::new();

        Self {
            materials,
            click_material,
            click_vec,
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
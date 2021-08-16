use crate::render::material::*;

use std::rc::Rc;

static mut MATERIAL_MANAGER: Option<MaterialManager> = None;

pub fn initialize_material_manager(display: &dyn glium::backend::Facade) {
    unsafe {
        assert!(MATERIAL_MANAGER.is_none()); // make sure this isn't called more than once
    
        MATERIAL_MANAGER = Some(MaterialManager::load_all(display));
    }
}

pub fn get_material_manager() -> &'static MaterialManager {
    unsafe {
        return &MATERIAL_MANAGER.as_ref().unwrap();
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum MaterialType {
    SolidColorMaterial,
    ClickMaterial,
}

pub struct MaterialManager {
    click_material: ClickMaterial,
    solid_color_material: SolidColorMaterial,
}

impl MaterialManager {
    pub fn get_click_material(&self) -> ClickMaterial {
        let material = self.click_material.clone();

        material
    }

    pub fn get_solid_color_material(&self) -> SolidColorMaterial {
        self.solid_color_material.clone()
    }

    pub fn load_all(display: &dyn glium::backend::Facade) -> Self {

        let click_material = ClickMaterial::new(Self::load_from_file(std::path::Path::new("./shaders/picker"), display), [1.0, 1.0, 1.0, 1.0]);
        let solid_color_material = SolidColorMaterial::new(Self::load_from_file(std::path::Path::new("./shaders/solid_color"), display));

        Self {
            click_material,
            solid_color_material
        }
    }

    fn load_from_file(path: &std::path::Path, display: &dyn glium::backend::Facade) -> Rc<glium::Program>{
        use std::io::Read;

        let mut fragment_file = std::fs::File::open(path.join("fragment.glsl")).expect(
            format!("unable to find file {}", std::env::current_dir().unwrap().join(path).join("fragment.glsl").into_os_string().to_str().unwrap()).as_str()
        );
        
        let mut vertex_file = std::fs::File::open(path.join("vertex.glsl")).expect(
            format!("unable to find file {}", std::env::current_dir().unwrap().join(path).join("vertex.glsl").into_os_string().to_str().unwrap()).as_str()
        );

        let mut fragment_shader_src = String::new();
        fragment_file.read_to_string(&mut fragment_shader_src).unwrap();

        let mut vertex_shader_src = String::new();
        vertex_file.read_to_string(&mut vertex_shader_src).unwrap();

        Rc::new(glium::Program::from_source(display, vertex_shader_src.as_str(), fragment_shader_src.as_str(), None).expect(format!{"Error compiling shader: {}", path.to_str().unwrap()}.as_str()))
    }
}
use crate::renderer::vertex::Vertex;
use crate::renderer::matrix::{WorldTransform};

use std::rc::Rc;

pub trait Material {
    fn draw(&self, shape: &(&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>), surface: &mut impl glium::Surface, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>);
}


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
    FxaaMaterial,
    MsaaMaterial,
}

pub struct MaterialManager {
    click_material: ClickMaterial,
    solid_color_material: SolidColorMaterial,
    fxaa_material: FxaaMaterial,
    msaa_material: MsaaMaterial,
}

impl MaterialManager {
    pub fn get_click_material(&self) -> ClickMaterial {
        let material = self.click_material.clone();

        material
    }

    pub fn get_solid_color_material(&self) -> SolidColorMaterial {
        self.solid_color_material.clone()
    }

    pub fn get_fxaa_material(&self) -> FxaaMaterial {
        self.fxaa_material.clone()
    }

    pub fn get_msaa_material(&self) -> MsaaMaterial {
        self.msaa_material.clone()
    }

    pub fn load_all(display: &dyn glium::backend::Facade) -> Self {

        let click_material = ClickMaterial::new(Self::load_from_file(std::path::Path::new("./shaders/picker"), display), [1.0, 1.0, 1.0, 1.0]);
        let solid_color_material = SolidColorMaterial::new(Self::load_from_file(std::path::Path::new("./shaders/solid_color"), display));
        let fxaa_material = FxaaMaterial::new(Self::load_from_file(std::path::Path::new("./shaders/fxaa"), display));
        let msaa_material = MsaaMaterial::new(Self::load_from_file(std::path::Path::new("./shaders/msaa"), display));

        Self {
            click_material,
            solid_color_material,
            fxaa_material,
            msaa_material,
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

#[derive(Clone)]
pub struct SolidColorMaterial {
    pub shader: Rc<glium::Program>,
}

impl SolidColorMaterial {
    pub fn new(shader: Rc<glium::Program>) -> Self {
        Self {
            shader,
        }
    }

    pub fn create_from_existing(&self) -> Self {
        Self {
            shader: self.shader.clone(),
        }
    }
    
    pub fn as_any(&self) -> Box<&dyn std::any::Any> {
        Box::new(self)
    }

    pub fn as_any_mut(&mut self) -> Box<&mut dyn std::any::Any> {
        Box::new(self)
    }

    pub fn to_any(self) -> Box<dyn std::any::Any> {
        Box::new(self)
    }

    pub fn draw(&self, shape: &(&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>), surface: &mut impl glium::Surface, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>) {
        // let uniforms = world_transform.to_uniform().add("tex", crate::render::textures::get_texture_array());

        if let Some(texture_array) = crate::renderer::textures::get_texture_array() {
            let uniforms = world_transform.to_uniform().add("tex", texture_array);
            crate::renderer::shape::draw(shape, surface, &self.shader, &uniforms, draw_parameters);
        } else {
            let uniforms = world_transform.to_uniform();
            crate::renderer::shape::draw(shape, surface, &self.shader, &uniforms, draw_parameters);
        }
        
    }

    pub fn get_shader_type(&self) -> MaterialType {
        MaterialType::SolidColorMaterial
    }
}

#[derive(Clone)]
pub struct ClickMaterial {
    pub shader: Rc<glium::Program>,
    pub color: [f32; 4],
}

impl ClickMaterial {
    pub fn new(shader: Rc<glium::Program>, color: [f32;4]) -> Self {
        Self {
            shader,
            color,
        }
    }

    pub fn create_from_existing(&self, new_color: [f32; 4]) -> Self {
        Self {
            shader: self.shader.clone(),
            color: new_color,
        }
    }
    
    pub fn as_any(&self) -> Box<&dyn std::any::Any> {
        Box::new(self)
    }

    pub fn as_any_mut(&mut self) -> Box<&mut dyn std::any::Any> {
        Box::new(self)
    }

    pub fn to_any(self) -> Box<dyn std::any::Any> {
        Box::new(self)
    }

    pub fn draw(&self, shape: &(&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>), surface: &mut impl glium::Surface, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>) {
        let uniforms = world_transform.to_uniform();
        
        crate::renderer::shape::draw(shape, surface, &self.shader, &uniforms, draw_parameters);
    }

    pub fn get_shader_type(&self) -> MaterialType {
        MaterialType::ClickMaterial
    }
}

#[derive(Clone)]
pub struct FxaaMaterial {
    pub shader: Rc<glium::Program>,
}

impl FxaaMaterial {
    pub fn new(shader: Rc<glium::Program>) -> Self {
        Self {
            shader,
        }
    }

    pub fn draw(&self, shape: &(&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>), surface: &mut impl glium::Surface, from_surface: &glium::texture::Texture2d, enabled: bool, draw_parameters: &glium::DrawParameters<'_>) {
        let target_dimensions = surface.get_dimensions();

        let uniforms = uniform! {
            tex: from_surface,
            enabled: if enabled {1i32} else {0i32},
            resolution: (target_dimensions.0 as f32, target_dimensions.1 as f32)
        };
        
        surface.draw(shape.0, shape.1, &self.shader, &uniforms, draw_parameters).unwrap();
    }
}

#[derive(Clone)]
pub struct MsaaMaterial {
    pub shader: Rc<glium::Program>,
}

impl MsaaMaterial {
    pub fn new(shader: Rc<glium::Program>) -> Self {
        Self {
            shader,
        }
    }

    pub fn draw(&self, shape: &(&glium::VertexBuffer<Vertex>, &glium::IndexBuffer<u32>), surface: &mut impl glium::Surface, from_surface: &glium::texture::Texture2dMultisample, samples: i32, draw_parameters: &glium::DrawParameters<'_>) {
        let target_dimensions = surface.get_dimensions();

        let uniforms = uniform! {
            tex: from_surface,
            samples: samples,
            resolution: (target_dimensions.0 as f32, target_dimensions.1 as f32)
        };
        
        surface.draw(shape.0, shape.1, &self.shader, &uniforms, draw_parameters).unwrap();
    }
}
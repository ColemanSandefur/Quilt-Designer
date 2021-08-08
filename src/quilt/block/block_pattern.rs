use crate::render::shape_object::{ShapeDataStruct};
use crate::glium::Surface;
use crate::parse::{Yaml, SavableBlueprint};

#[derive(Clone)]
pub struct BlockPattern {
    shapes: Vec<Box<ShapeDataStruct>>,
    texture_id: Option<imgui::TextureId>,
    pattern_name: String,
}

impl BlockPattern {
    pub fn new(mut shapes: Vec<Box<ShapeDataStruct>>, name: String) -> Self {

        // add square to background and black outline to square pattern

        shapes.insert(0, 
            Box::new(ShapeDataStruct::new(
                Box::new(crate::render::shape::Square::with_line_width(0.0, 0.0, 1.0, 1.0, 0, 0.0)),
            )),
        );

        shapes.push(
            Box::new(ShapeDataStruct::new(
                Box::new(crate::render::shape::StrokeShape::square(0.0, 0.0, 1.0, 1.0, 0, &lyon::lyon_tessellation::StrokeOptions::default().with_line_width(crate::quilt::block::Block::BLOCK_BORDER_WIDTH)),
            )),
        ));

        Self {
            shapes,
            texture_id: None,
            pattern_name: name,
        }
    }

    pub fn get_shapes(&self) -> &Vec<Box<ShapeDataStruct>> {
        &self.shapes
    }

    pub fn get_shape_clone(&self) -> Vec<Box<ShapeDataStruct>> {
        let mut vec = Vec::with_capacity(self.shapes.len());

        for shape in &self.shapes {
            vec.push(shape.clone());
        }

        vec
    }

    pub fn get_pattern_name(&self) -> &String {
        &self.pattern_name
    }

    pub fn get_texture_id(&self) -> &Option<imgui::TextureId> {
        &self.texture_id
    }

    pub fn draw(&self, surface: &mut impl glium::Surface, facade: & impl glium::backend::Facade) {

        // get num elements to avoid resizing vector
        let mut total_vertices = 0;
        let mut total_indices = 0;

        for shape in &self.shapes {
            total_vertices = shape.shape.get_num_vertices();
            total_indices = shape.shape.get_num_indices();
        }

        let mut vb_vec: Vec<crate::render::shape::Vertex> = Vec::with_capacity(total_vertices);
        let mut ib_vec = Vec::with_capacity(total_indices);

        for shape in &self.shapes {
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

    pub fn create_and_draw_texture(&mut self, display: &impl glium::backend::Facade, textures: &mut imgui::Textures<imgui_glium_renderer::Texture>) {

        let texture = glium::texture::Texture2d::empty(
            display,
            512,
            512
        ).unwrap();

        let mut surface = texture.as_surface();

        surface.clear_color(0.0, 0.0, 0.0, 1.0);

        self.draw(&mut surface, display);

        let mut sampler = glium::uniforms::SamplerBehavior::default();
        sampler.magnify_filter = glium::uniforms::MagnifySamplerFilter::Linear;
        sampler.minify_filter = glium::uniforms::MinifySamplerFilter::LinearMipmapLinear;
        sampler.max_anisotropy = 65535;

        let texture_id = textures.insert(imgui_glium_renderer::Texture
            {
                texture: std::rc::Rc::new(texture),
                sampler: Default::default()
            }
        );


        self.texture_id = Some(texture_id);
    }
}

impl SavableBlueprint for BlockPattern {
    fn from_save_blueprint(yaml: Yaml) -> Box<Self> where Self: Sized {
        let yaml_vec = Into::<Vec<_>>::into(yaml);

        let mut shapes = Vec::with_capacity(yaml_vec.len());
        
        for yaml_entry in yaml_vec {
            shapes.push(ShapeDataStruct::from_save_blueprint(yaml_entry));
        }

        Box::new(Self {
            shapes,
            texture_id: None,
            pattern_name: String::from(""),
        })
    }
}

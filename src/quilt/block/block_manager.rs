use crate::render::shape_object::ShapeDataStruct;
use crate::quilt::block::block_pattern::BlockPattern;
use crate::parse::Yaml;
use crate::parse::SavableBlueprint;

use lazy_static::lazy_static;
use std::io::Read;
use std::sync::Mutex;

fn is_pattern(extension: &std::ffi::OsStr) -> bool {
    let path = extension.to_str().unwrap().to_lowercase();

    match path {
        p if p.eq("yaml") => true,
        _ => false
    }
}

fn load_pattern_yaml(path: &std::path::Path) -> BlockPattern {
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let yaml = Yaml::load_from_file(path);

    *BlockPattern::from_save_blueprint(yaml)
}

fn load_patterns() -> Vec<BlockPattern> {
    let path = std::path::Path::new("./patterns");
    let mut patterns = Vec::with_capacity(10);

    if path.is_dir() {
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let file_name = entry.file_name();
            let extension = std::path::Path::new(&file_name).extension().unwrap();

            if is_pattern(extension) {
                patterns.push(load_pattern_yaml(&entry.path()));
            }
        }
    }

    patterns
}

lazy_static!{
    // Will load block designs from a dedicated folder
    pub static ref BLOCK_LIST: Mutex<Vec<BlockPattern>> = Mutex::new(
        
        {
            let mut default_blocks = vec! {
                BlockPattern::new(vec![
                    Box::new(
                        ShapeDataStruct::new(
                            Box::new(crate::render::shape::PathShape::triangle((0.0, 0.0), (0.0, 1.0), (1.0, 0.0), 0)),
                        )
                    ),
                ], String::from("half-square triangle")),
        
                BlockPattern::new(vec![
                    Box::new(
                        ShapeDataStruct::new(
                            Box::new(crate::render::shape::PathShape::square(0.0, 0.0, 1.0, 1.0, 0)),
                        )
                    )
                ], String::from("square")),
        
                BlockPattern::new(vec![
                    Box::new(ShapeDataStruct::new(
                        Box::new(crate::render::shape::PathShape::square(0.25, 0.25, 0.5, 0.5, 0)),
                    )),
                    Box::new(ShapeDataStruct::new(
                        Box::new(crate::render::shape::PathShape::square(0.3, 0.3, 0.4, 0.4, 0)),
                    )),
                    Box::new(ShapeDataStruct::new(
                        Box::new(crate::render::shape::PathShape::square(0.35, 0.35, 0.3, 0.3, 0)),
                    )),
                    Box::new(ShapeDataStruct::new(
                        Box::new(
                            crate::render::shape::PathShape::circle(lyon::math::point(0.5, 0.5), 0.25, -0.5 * std::f32::consts::PI, 0.5 * std::f32::consts::PI, 0),
                        ),
                    )),
                ], String::from("test shape")),
            };

            default_blocks.append(&mut load_patterns());

            default_blocks
        }
    );
}

// Generate the imgui icons for each texture
pub fn load_textures(display: &impl glium::backend::Facade, glium_renderer: &mut imgui_glium_renderer::Renderer) {

    let mut textures = glium_renderer.textures();

    let mut block_list = BLOCK_LIST.lock().unwrap();

    for square_pattern in block_list.iter_mut() {
        square_pattern.create_and_draw_texture(display, &mut textures);
    }
}
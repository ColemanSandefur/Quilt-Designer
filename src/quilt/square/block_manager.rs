use crate::render::object::ShapeDataStruct;
use crate::quilt::square::square_pattern::SquarePattern;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static!{
    pub static ref BLOCK_LIST: Mutex<Vec<SquarePattern>> = Mutex::new(vec!{
        SquarePattern::new(vec![
            Box::new(
                ShapeDataStruct::new(
                    Box::new(crate::render::shape::Triangle::new((0.0, 0.0), (0.0, 1.0), (1.0, 0.0), 0)),
                )
            ),
        ], String::from("half-square triangle")),

        SquarePattern::new(vec![
            Box::new(ShapeDataStruct::new(
                Box::new(crate::render::shape::Square::with_width_height(0.25, 0.25, 0.5, 0.5, 0)),
            )),
            Box::new(ShapeDataStruct::new(
                Box::new(crate::render::shape::Square::with_width_height(0.3, 0.3, 0.4, 0.4, 0)),
            )),
            Box::new(ShapeDataStruct::new(
                Box::new(crate::render::shape::Square::with_width_height(0.35, 0.35, 0.3, 0.3, 0)),
            )),
            Box::new(ShapeDataStruct::new(
                Box::new(
                    crate::render::shape::PathShape::circle(lyon::math::point(0.5, 0.5), 0.25, -0.5 * std::f32::consts::PI, 0.5 * std::f32::consts::PI, 0),
                ),
            )),
        ], String::from("test shape")),

    });
}

pub fn load_textures(display: &impl glium::backend::Facade, glium_renderer: &mut imgui_glium_renderer::Renderer) {
    let mut textures = glium_renderer.textures();

    let mut block_list = BLOCK_LIST.lock().unwrap();

    for square_pattern in block_list.iter_mut() {
        square_pattern.create_and_draw_texture(display, &mut textures);
    }
}
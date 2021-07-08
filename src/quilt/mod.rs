pub mod square;

use crate::render::material::{material_manager::{MaterialManager}};
use crate::render::matrix::{WorldTransform};
use crate::render::object::ShapeObject;
use square::Square;

#[allow(dead_code)]
pub struct Quilt {
    width: usize,
    height: usize,
    squares: Vec<Vec<Square>>
}

impl Quilt {
    pub fn new(display: &dyn glium::backend::Facade, shaders: &MaterialManager, width: usize, height: usize) -> Self {

        let mut squares = Vec::with_capacity(height);

        for r in 0..height {
            let mut row = Vec::with_capacity(width);

            for c in 0..width {
                let mut square = Square::new(display, shaders);

                let column = c as f32;
                let r = -1.0 * r as f32 - 1.0;

                square.model_transform.translate(column - width as f32 / 2.0, r + height as f32 / 2.0, 0.0);
                row.push(square);
            }

            squares.push(row);
        }

        Self {
            width,
            height,
            squares,
        }
    }

    pub fn draw(&mut self, frame: &mut glium::Frame, world_transform: &WorldTransform, draw_parameters: &glium::DrawParameters<'_>) {
        for row in &mut self.squares {
            for square in row {
                square.draw(frame, world_transform, draw_parameters);
            }
        }
    }
}
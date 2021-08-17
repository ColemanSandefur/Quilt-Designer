pub mod brush;
pub mod block;

use crate::program::quilt::brush::*;
use crate::renderer::picker::{Picker, PickerEntry};
use crate::renderer::{Renderable, Renderer, RenderToken};
use block::Block;

#[allow(dead_code)]
pub struct Quilt {
    pub width: usize,
    pub height: usize,
    pub squares: Vec<Vec<Block>>,
    needs_updated: bool,
    renderer_id: Option<RenderToken>,
}

impl Quilt {

    pub const INIT_VERTICES: usize = 6000;
    pub const INIT_INDICES: usize = Self::INIT_VERTICES * 4;

    pub fn new(width: usize, height: usize, picker: &mut Picker) -> Self {
        let mut squares = Vec::with_capacity(height);

        for r in 0..height {
            let mut row = Vec::with_capacity(width);

            for c in 0..width {
                let mut square = Block::new(r, c, picker);

                let column = c as f32;
                let r = -1.0 * r as f32 - 1.0;

                let mut transform = square.get_model_transform();
                
                transform.translate(column - width as f32 / 2.0, r + height as f32 / 2.0, 0.0);

                square.set_model_transform(transform);
                row.push(square);
            }

            squares.push(row);
        }

        println!("Finished loading squares");

        Self {
            width,
            height,
            squares,
            needs_updated: true,
            renderer_id: None,
        }
    }

    pub fn calc_offset(&self, row: usize, column: usize) -> (f32, f32) {
        let column = column as f32;
        let r = -1.0 * row as f32 - 1.0;

        (column - self.width as f32 / 2.0, r + self.height as f32 / 2.0)
    }
    
    pub fn draw(&mut self, renderer: &mut Renderer) {

        // Whenever we change the shape's data, we need to give the renderer the new information for it to render
        if self.needs_updated {
            let mut render_items: Vec<Box<dyn Renderable>> = Vec::with_capacity(self.width * self.height);

            for row in &mut self.squares {
                for block in row {
                    render_items.push(Box::new(block.clone()))
                }
            }
            
            if self.renderer_id.is_none() {
                self.renderer_id = Some(renderer.add_render_items(render_items));
            } else {
                renderer.set_render_items(render_items, self.renderer_id.unwrap());
            }

            self.needs_updated = false;
        }
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn click(&mut self, entry: &PickerEntry, brush: &Brush, picker: &mut Picker) -> bool {

        if self.squares[entry.row][entry.column].click(entry.id, brush, picker) {
            self.needs_updated = true;

            return true;
        }


        false
    }
}
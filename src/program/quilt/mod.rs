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
    blocks: Vec<Vec<Block>>,
    needs_updated: bool,
    renderer_id: Option<RenderToken>,
}

impl Quilt {
    pub fn new(width: usize, height: usize, picker: &mut Picker) -> Self {
        let mut blocks = Vec::with_capacity(height);

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

            blocks.push(row);
        }

        println!("Finished loading squares");

        Self {
            width,
            height,
            blocks,
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

            for row in &mut self.blocks {
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

        if self.blocks[entry.row][entry.column].click(entry.id, brush, picker) {
            self.needs_updated = true;

            return true;
        }


        false
    }

    // automatically gets the row and column from Block
    pub fn set_block(&mut self, block: Block) {
        let (row, column) = (block.get_row(), block.get_column());
        let offset = self.calc_offset(row, column);
        
        let mut model_transform = block.get_model_transform();
        model_transform.translate(offset.0, offset.1, 0.0);
        
        self.blocks[row][column] = block;
        self.blocks[row][column].set_model_transform(model_transform);

        self.needs_updated = true;
    }

    pub fn get_block(&self, row: usize, column: usize) -> &Block {
        &self.blocks[row][column]
    }
}
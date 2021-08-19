pub mod brush;
pub mod block;

use crate::parse::*;
use crate::program::quilt::brush::*;
// use crate::renderer::picker::{Picker, PickerEntry};
use crate::renderer::new_picker::*;
use crate::renderer::{Renderable, Renderer, RenderToken};
use block::Block;

use std::sync::{Arc, Mutex};

#[allow(dead_code)]
pub struct Quilt {
    pub width: usize,
    pub height: usize,
    blocks: Vec<Vec<Block>>,
    needs_updated: Arc<Mutex<bool>>,
    renderer_id: Option<RenderToken>,
}

impl Quilt {
    pub fn new(width: usize, height: usize, picker: &mut Picker, brush: Arc<Mutex<Brush>>) -> Self {
        let mut blocks = Vec::with_capacity(height);
        let needs_updated = Arc::new(Mutex::new(true));

        for r in 0..height {
            let mut row = Vec::with_capacity(width);

            for c in 0..width {
                let mut square = Block::new(r, c, picker, needs_updated.clone(), brush.clone());

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
            needs_updated,
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
        if *self.needs_updated.lock().unwrap() {
            println!("Updating");
            let mut render_items: Vec<Box<dyn Renderable>> = Vec::with_capacity(self.width * self.height);

            for row in &mut self.blocks {
                for block in row {
                    block.update();
                    render_items.push(Box::new(block.clone()))
                }
            }
            
            if self.renderer_id.is_none() {
                self.renderer_id = Some(renderer.get_render_items_mut().borrow_mut().add_render_items(render_items));
            } else {
                renderer.get_render_items_mut().borrow_mut().set_render_items(render_items, self.renderer_id.as_ref().unwrap().clone());
            }

            *self.needs_updated.lock().unwrap() = false;
        }
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    // pub fn click(&mut self, entry: &PickerEntry, brush: &Brush, picker: &mut Picker) -> bool {

    //     if self.blocks[entry.row][entry.column].click(entry.id, brush, picker) {
    //         self.needs_updated = true;

    //         return true;
    //     }


    //     false
    // }

    // automatically gets the row and column from Block
    pub fn set_block(&mut self, block: Block) {
        let (row, column) = (block.get_row(), block.get_column());
        let offset = self.calc_offset(row, column);
        
        let mut model_transform = block.get_model_transform();
        model_transform.translate(offset.0, offset.1, 0.0);
        
        self.blocks[row][column] = block;
        self.blocks[row][column].set_model_transform(model_transform);

        *self.needs_updated.lock().unwrap() = true;
    }

    pub fn get_block(&self, row: usize, column: usize) -> &Block {
        &self.blocks[row][column]
    }

    pub fn to_save(&self, save_data: &mut SaveData) -> Yaml {
        let mut output_vec: Vec<Yaml> = Vec::with_capacity(self.width * self.height);

        for row in &self.blocks {
            for block in row {
                output_vec.push(block.to_save(save_data));
            }
        }

        LinkedHashMap::create(vec![
            ("quilt", Yaml::from(output_vec)),
            ("width", self.width.into()),
            ("height", self.height.into()),
        ])
    }

    pub fn from_save(&self, yaml: Yaml, picker: &mut Picker, brush: Arc<Mutex<Brush>>, save_data: &mut SaveData) -> Self {
        let yaml_map = LinkedHashMap::from(yaml);

        let quilt_yaml = Vec::<Yaml>::from(yaml_map.get("quilt"));

        let (width, height) = (yaml_map.get("width").into(), yaml_map.get("height").into());

        let mut quilt = Self::new(width, height, picker, brush.clone());
        let needs_updated = Arc::new(Mutex::new(true));

        for block_yaml in quilt_yaml {
            let block = Block::from_save(block_yaml, picker, needs_updated.clone(), brush.clone(), save_data);

            quilt.set_block(block);
        }

        quilt
    }
}
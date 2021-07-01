use crate::draw::texture_brush::TextureBrush;
use crate::quilt::block_pattern::BlockPattern;

use std::sync::{Arc};

pub struct Brush {
    texture_brush: Option<Arc<TextureBrush>>,
    block_pattern: Option<BlockPattern>,
}

impl Brush {
    pub fn with_texture(brush: Arc<TextureBrush>) -> Self {
        Self {
            texture_brush: Some(brush),
            block_pattern: None,
        }
    }

    pub fn set_texture(&mut self, brush: Arc<TextureBrush>) {
        self.texture_brush = Some(brush);
        self.block_pattern = None;
    }

    pub fn get_texture(&self) -> Option<Arc<TextureBrush>> {
        match &self.texture_brush {
            Some(b) => Some(b.clone()),
            _ => None
        }
    }

    pub fn is_texture_brush(&self) -> bool {
        match self.texture_brush {
            Some(_) => true,
            _ => false,
        }
    }

    pub fn set_block_pattern(&mut self, brush: BlockPattern) {
        self.texture_brush = None;
        self.block_pattern = Some(brush);
    }

    pub fn get_block_pattern(&self) -> Option<BlockPattern> {
        match &self.block_pattern {
            Some(b) => Some(b.clone()),
            _ => None
        }
    }

    pub fn is_block_pattern_brush(&self) -> bool {
        match self.block_pattern {
            Some(_) => true,
            _ => false,
        }
    }
}
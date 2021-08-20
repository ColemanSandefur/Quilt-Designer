use std::sync::Arc;
use crate::program::quilt::block::block_pattern::BlockPattern;

static mut ROTATION: f32 = 0.0;

pub struct Brush {
    block_brush: Option<Arc<BlockBrush>>,
    pattern_brush: Option<Arc<PatternBrush>>,
}

impl Brush {
    pub fn set_rotation(rotation: f32) {
        unsafe {ROTATION = rotation % (2.0 * std::f32::consts::PI);}
    }

    pub fn get_rotation() -> f32 {
        unsafe {ROTATION}
    }

    pub fn increase_rotation(rotation: f32) {
        unsafe {Self::set_rotation(rotation + ROTATION);}
    }

    pub fn new_block_brush(block_brush: BlockBrush) -> Self {
        Self {
            block_brush: Some(Arc::new(block_brush)),
            pattern_brush: None,
        }
    }

    pub fn new_pattern_brush(pattern_brush: PatternBrush) -> Self {
        Self {
            block_brush: None,
            pattern_brush: Some(Arc::new(pattern_brush)),
        }
    }

    pub fn set_block_brush(&mut self, block_brush: Arc<BlockBrush>) {
        self.block_brush = Some(block_brush);
        self.pattern_brush = None;
    }

    pub fn set_pattern_brush(&mut self, pattern_brush: Arc<PatternBrush>) {
        self.block_brush = None;
        self.pattern_brush = Some(pattern_brush);
    }

    pub fn get_block_brush(&self) -> Option<Arc<BlockBrush>> {
        self.block_brush.clone()
    }

    pub fn get_pattern_brush(&self) -> Option<Arc<PatternBrush>> {
        self.pattern_brush.clone()
    }

    pub fn is_block_brush(&self) -> bool {
        self.block_brush.is_some()
    }

    pub fn is_pattern_brush(&self) -> bool {
        self.pattern_brush.is_some()
    }
}

pub struct BlockBrush {
    pub square_pattern: BlockPattern,
}

impl BlockBrush {
    pub fn new(square_pattern: BlockPattern) -> Self {
        Self {
            square_pattern,
        }
    }

    pub fn get_pattern(&self) -> BlockPattern {
        let mut block_pattern = self.square_pattern.clone();

        let shapes = block_pattern.get_mut_shapes();

        // skip last shape because it is just the block border, and I don't want it to have an id or change it's color
        for index in 0..shapes.len()-1 {
            let shape = &mut (*shapes[index]).shape;

            shape.set_color([1.0; 4]);
            unsafe {shape.set_rotation(ROTATION);}
        }

        block_pattern
    }
}

pub struct PatternBrush {
    color: Option<[f32; 4]>,
    texture: Option<crate::renderer::textures::Texture>,
}

impl PatternBrush {
    pub fn new_color(color: [f32; 4]) -> Self {
        Self {
            color: Some(color),
            texture: None,
        }
    }

    pub fn new_texture(texture: crate::renderer::textures::Texture) -> Self {
        Self {
            color: None,
            texture: Some(texture),
        }
    }

    pub fn get_color(&self) -> &Option<[f32; 4]> {
        &self.color
    }

    pub fn get_texture(&self) -> &Option<crate::renderer::textures::Texture> {
        &self.texture
    }

    pub fn apply_to_shape(&self, shape: &mut crate::renderer::shape_object::ShapeDataStruct) {
        if let Some(color) = self.color.as_ref() {
            shape.shape.set_color(*color);
        } else if let Some(texture) = self.texture.as_ref() {
            // increase the index by 1 because 0 is used as a "no texture" in the vertex
            shape.shape.set_tex_id(texture.get_texture_index() as u32 + 1);
        }
    }
}
use std::sync::Arc;
use crate::quilt::square::square_pattern::SquarePattern;

pub struct Brush {
    block_brush: Option<Arc<BlockBrush>>,
    pattern_brush: Option<Arc<PatternBrush>>,
}

impl Brush {
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
    pub square_pattern: SquarePattern
}

impl BlockBrush {
    pub fn new(square_pattern: SquarePattern) -> Self {
        Self {
            square_pattern
        }
    }

    pub fn get_pattern(&self, picker: &mut crate::render::picker::Picker, row: usize, column: usize) -> SquarePattern {
        let mut shapes = self.square_pattern.get_shapes().clone();

        for shape in &mut shapes {
            let shape = &mut (*shape).shape;

            shape.set_id(picker.get_new_id(row, column));
            shape.set_color([1.0; 4]);
        }

        SquarePattern::new(shapes)
    }
}

pub struct PatternBrush {
    pub color: [f32; 4]
}
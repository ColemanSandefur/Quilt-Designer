use crate::quilt::child_shape::ChildShape;
use crate::path::{Line, Move};
use std::sync::{Arc};

pub fn create_rect(x: f64, y: f64, width: f64, height: f64) -> ChildShape {
    ChildShape::new_with_paths(vec![
        Arc::new(Move::new(x, y)),
        Arc::new(Line::new(x + width, y)),
        Arc::new(Line::new(x + width, y + height)),
        Arc::new(Line::new(x, y + height)),
        Arc::new(Line::new(x, y)),
    ])
}
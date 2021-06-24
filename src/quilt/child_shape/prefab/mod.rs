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

pub fn create_triangle(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> ChildShape {
    ChildShape::new_with_paths(vec![
        Arc::new(Move::new(p1.0, p1.1)),
        Arc::new(Line::new(p2.0, p2.1)),
        Arc::new(Line::new(p3.0, p3.1)),
    ])
}
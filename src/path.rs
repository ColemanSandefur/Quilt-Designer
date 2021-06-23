use cairo::{Context};
use std::sync::{Arc};

pub trait Path: std::marker::Sync + std::marker::Send {
    fn draw_path(&self, cr: &Context);
    fn clone_path(&self) -> Arc<dyn Path>;
}

///////////////////////////////////////////////////////////////
////      Line      ///////////////////////////////////////////
///////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Line {
    end: (f64, f64),
}

impl Line {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            end: (x,y)
        }
    }
}

impl Path for Line {
    fn draw_path(&self, cr: &Context) {
        cr.line_to(self.end.0, self.end.1);
    }

    fn clone_path(&self) -> Arc<dyn Path> {
        Arc::new(self.clone())
    }
}

///////////////////////////////////////////////////////////////
////      Move      ///////////////////////////////////////////
///////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Move {
    point: (f64, f64),
}

impl Move {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            point: (x, y)
        }
    }
}

impl Path for Move {
    fn draw_path(&self, cr: &Context) {
        cr.move_to(self.point.0, self.point.1);
    }

    fn clone_path(&self) -> Arc<dyn Path> {
        Arc::new(self.clone())
    }
}

///////////////////////////////////////////////////////////////
////      Arc       ///////////////////////////////////////////
///////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct ArcPath {
    center: (f64, f64),
    radius: f64,
    start_angle: f64,
    end_angle: f64,
}

impl ArcPath {
    pub fn new(xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) -> Self {
        Self {
            center: (xc, yc),
            radius,
            start_angle: angle1,
            end_angle: angle2
        }
    }
}

impl Path for ArcPath {
    fn draw_path(&self, cr: &Context) {
        cr.arc(self.center.0, self.center.1, self.radius, self.start_angle, self.end_angle);
    }

    fn clone_path(&self) -> Arc<dyn Path> {
        Arc::new(self.clone())
    }
}
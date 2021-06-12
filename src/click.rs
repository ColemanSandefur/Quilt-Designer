use gdk::{EventButton};
use cairo::Context;

pub trait Click {
    // returns whether or not it was actually clicked
    fn click(&mut self, cr: &Context, location: &EventButton) -> bool;
    // fn click(&mut self, cr: &Context, location: (f64, f64)) -> bool;
}
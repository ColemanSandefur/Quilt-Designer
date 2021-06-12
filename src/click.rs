use crate::canvas::Canvas;
use gdk::{EventButton};
use cairo::Context;

pub trait Click {
    // returns whether or not it was actually clicked
    fn click(&mut self, window: &Canvas, cr: &Context, location: &EventButton) -> bool;
}
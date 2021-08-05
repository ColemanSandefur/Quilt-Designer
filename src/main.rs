#[macro_use]
extern crate glium;
pub mod quilt;
pub mod render;
pub mod system;
pub mod util;

fn main() {
    let system = system::init("title");

    system.main_loop(move |_, frame, renderer, ui, glium_renderer, facade| {
        renderer.draw(frame, ui, glium_renderer, facade);
    });
}

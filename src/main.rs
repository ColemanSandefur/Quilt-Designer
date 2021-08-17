#[macro_use]
extern crate glium;
pub mod program;
pub mod renderer;
pub mod system;
pub mod parse;

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let mut system = system::init("Quilt Designer");

    crate::renderer::material::initialize_material_manager(&*system.display);
    crate::renderer::textures::load_texture_array(&*system.display, &mut system.glium_renderer.textures());
    crate::program::quilt::block::block_manager::load_textures(&*system.display, &mut system.glium_renderer);

    let draw_program = Rc::new(RefCell::new(crate::program::Program::new(system.display.clone())));
    let window_program = draw_program.clone();

    system.main_loop(move |_, frame, ui, _glium_renderer, _facade| {
        draw_program.borrow_mut().draw(frame, ui);
    }, move |event| {
        window_program.borrow_mut().window_event(event);
    });
}

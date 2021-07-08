#[macro_use]
extern crate glium;
mod render;
mod system;
mod util;

fn main() {
    let system = system::init("title");

    system.main_loop(move |_, frame, renderer, ui| {
        // println!("{}ms", renderer.frame_timing.delta_frame_time().num_milliseconds());

        renderer.draw(frame, ui);
    });
}

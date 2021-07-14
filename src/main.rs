#[macro_use]
extern crate glium;
pub mod quilt;
pub mod render;
pub mod system;
pub mod util;

fn main() {
    let system = system::init("title");

    system.main_loop(move |_, frame, renderer, ui| {
        // println!("{}ms", renderer.frame_timing.delta_frame_time().num_milliseconds());
        // println!("{:?}", renderer.world_transform.get_scale());

        renderer.draw(frame, ui);
    });
}

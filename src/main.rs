#[macro_use]
extern crate glium;
pub mod quilt;
pub mod render;
pub mod system;
pub mod util;

fn main() {
    let mut system = system::init("title");

    system.renderer.world_transform.set_scale(1.0, 1.0, 8.0);

    system.main_loop(move |_, frame, renderer, ui| {
        // println!("{}ms", renderer.frame_timing.delta_frame_time().num_milliseconds());
        // println!("{:?}", renderer.world_transform.get_scale());

        renderer.draw(frame, ui);
    });
}

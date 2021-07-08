#[macro_use]
extern crate glium;
mod render;
mod system;
mod util;

fn main() {
    let system = system::init("title");

    system.main_loop(move |_, frame, keyboard_tracker, renderer, ui| {
        use imgui::*;
        use glium::Surface;
        use glium::glutin::event::VirtualKeyCode;

        let dimensions = frame.get_dimensions();

        Window::new(im_str!("Textures"))
            .size([100.0, dimensions.1 as f32], Condition::Appearing)
            .size_constraints([100.0, dimensions.1 as f32], [dimensions.0 as f32, dimensions.1 as f32])
            .position([0.0, 0.0], Condition::Always)
            .bg_alpha(1.0)
            .movable(false)
            .collapsible(false)
            .build(ui, || {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("こんにちは世界！"));
                ui.text(im_str!("This...is...imgui-rs!"));
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
        
        Window::new(im_str!("Block Designs"))
            .size([100.0, dimensions.1 as f32], Condition::Appearing)
            .size_constraints([100.0, dimensions.1 as f32], [dimensions.0 as f32, dimensions.1 as f32])
            .position([dimensions.0 as f32, 0.0], Condition::Always)
            .position_pivot([1.0, 0.0])
            .bg_alpha(1.0)
            .movable(false)
            .collapsible(false)
            .build(ui, || {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("こんにちは世界！"));
                ui.text(im_str!("This...is...imgui-rs!"));
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
        
        // println!("{}ms", renderer.frame_timing.delta_frame_time().num_milliseconds());
        println!("{:?}", renderer.world_transform.get_translation());
        
        if keyboard_tracker.is_key_pressed(&VirtualKeyCode::A) {
            renderer.world_transform.translate(renderer.frame_timing.delta_frame_time().num_microseconds().unwrap() as f32 / 1_000.0 * 0.005, 0.0, 0.0);
        }
        if keyboard_tracker.is_key_pressed(&VirtualKeyCode::D) {
            renderer.world_transform.translate(renderer.frame_timing.delta_frame_time().num_microseconds().unwrap() as f32 / 1_000.0 * -0.005, 0.0, 0.0);
        }
        if keyboard_tracker.is_key_pressed(&VirtualKeyCode::Q) {
            renderer.world_transform.translate(0.0, 0.0, renderer.frame_timing.delta_frame_time().num_microseconds().unwrap() as f32 / 1_000.0 * -0.005);

            let translation = renderer.world_transform.get_translation();
            if translation.2 <= -0.7 {
                renderer.world_transform.set_translation(translation.0, translation.1, -0.7);
            }
        }
        if keyboard_tracker.is_key_pressed(&VirtualKeyCode::E) {
            renderer.world_transform.translate(0.0, 0.0, renderer.frame_timing.delta_frame_time().num_microseconds().unwrap() as f32 / 1_000.0 * 0.005);
        }
        
    }, |frame, renderer| {
        renderer.draw(frame);
    })
}

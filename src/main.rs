#[macro_use]
extern crate glium;
mod render;
mod system;

use render::renderer::Renderer;

fn main() {
    let system = system::init("title");

    system.main_loop(move |_, ui| {
        use imgui::*;
        Window::new(im_str!("Hello world"))
            .size([300.0, 110.0], Condition::FirstUseEver)
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
    }, |frame, renderer| {
        renderer.draw(frame);
    })
}

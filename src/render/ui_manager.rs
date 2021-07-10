use crate::render::renderer::Renderer;

pub struct UiManager {}

impl UiManager {

    pub fn draw(renderer: &mut Renderer, frame: &mut glium::Frame, ui: &mut imgui::Ui) {
        use imgui::*;
        use glium::Surface;

        let dimensions = frame.get_dimensions();

        Window::new(im_str!("Textures"))
            .size([100.0, dimensions.1 as f32], Condition::Appearing)
            .size_constraints([100.0, dimensions.1 as f32], [dimensions.0 as f32, dimensions.1 as f32])
            .position([0.0, 0.0], Condition::Always)
            .bg_alpha(1.0)
            .movable(false)
            .collapsible(false)
            .build(ui, || {
                ui.text(im_str!("{}ms", renderer.frame_timing.delta_frame_time().num_milliseconds()));
                ui.text(im_str!("{:.0} fps", 1.0 / (renderer.frame_timing.delta_frame_time().num_microseconds().unwrap() as f64 / 1_000_000.0)));
                ui.text(im_str!("drew {} times", renderer.quilt.draw_stats.draws));
                ui.text(im_str!("drew {} vertices", renderer.quilt.draw_stats.vertices));
                ui.text(im_str!("drew {} indices", renderer.quilt.draw_stats.indices));
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
    }
}
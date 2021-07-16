use crate::render::renderer::Renderer;

pub struct UiManager {}

impl UiManager {

    pub fn draw(renderer: &mut Renderer, frame: &mut glium::Frame, ui: &mut imgui::Ui) {
        use imgui::*;
        use glium::Surface;

        let dimensions = frame.get_dimensions();

        
        let mut did_click = false;
        let mut color = *renderer.brush;
        
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
                ui.text(im_str!("{} draws", renderer.quilt.draw_stats.draws));
                ui.text(im_str!("{} vertices", renderer.quilt.draw_stats.vertices));
                ui.text(im_str!("{} indices", renderer.quilt.draw_stats.indices));
                let picker = ColorPicker::new(im_str!("color picker"), &mut color);
                did_click = picker.build(&ui);
            });
        
        if did_click {
            renderer.brush = std::sync::Arc::new(color);
        }
        
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
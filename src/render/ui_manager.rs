use crate::render::renderer::Renderer;

struct ClickState {
    pub clicked: bool,
    pub double_clicked: bool,
}

static mut IS_COLOR_PICKER_OPEN: bool = false;
static mut COLOR_PICKER: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct UiManager {}

impl UiManager {

    pub fn draw(renderer: &mut Renderer, frame: &mut glium::Frame, ui: &mut imgui::Ui) {
        use imgui::*;
        use glium::Surface;

        let dimensions = frame.get_dimensions();

        let mut was_color_clicked = ClickState{ clicked: false, double_clicked: false };
        let color = unsafe {
            // *renderer.brush
            COLOR_PICKER
        };
        
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

                let button = imgui::ColorButton::new(im_str!("Color"), color)
                    .size([40.0, 40.0]);
                was_color_clicked.clicked = button.build(&ui);
                was_color_clicked.double_clicked = ui.is_item_hovered() && ui.is_mouse_double_clicked(MouseButton::Left);
                
                if was_color_clicked.clicked {
                    renderer.brush = std::sync::Arc::new(color)
                }

                if was_color_clicked.double_clicked {
                    unsafe {IS_COLOR_PICKER_OPEN = true;}
                }
            });

        unsafe {
            if IS_COLOR_PICKER_OPEN {
                Window::new(im_str!("Color Picker"))
                    // .size([200.0, 400.0], Condition::Appearing)
                    .opened(&mut IS_COLOR_PICKER_OPEN)
                    .always_auto_resize(true)
                    .build(ui, || {
                        let picker = ColorPicker::new(im_str!("color picker"), &mut COLOR_PICKER);
                        if picker.build(&ui) {
                            renderer.brush = std::sync::Arc::new(COLOR_PICKER)
                        }
                    });
                
            }
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

        
        // Checks if any imgui window was clicked, if not tell the renderer that it was clicked
        if ui.is_mouse_clicked(MouseButton::Left) {
            if !ui.is_window_hovered_with_flags(WindowHoveredFlags::all()) {
                renderer.clicked();
            }
        }
    }
}
use crate::quilt::brush::*;
use crate::render::renderer::Renderer;
use lazy_static::lazy_static;

struct ClickState {
    pub clicked: bool,
    pub double_clicked: bool,
}

static mut IS_COLOR_PICKER_OPEN: bool = false;
static mut COLOR_PICKER_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

lazy_static! {
    pub static ref UI_STYLE: Vec<(imgui::StyleColor, [f32; 4])> = vec! {
        (imgui::StyleColor::ResizeGrip, [0.0; 4]),
        (imgui::StyleColor::ResizeGripActive, [0.0; 4]),
        (imgui::StyleColor::ResizeGripHovered, [0.0; 4]),
        (imgui::StyleColor::Text, [1.0, 1.0, 1.0, 1.0]),
        (imgui::StyleColor::TitleBg, [0.2, 0.2, 0.2, 1.0]),
        (imgui::StyleColor::TitleBgActive, [0.2, 0.2, 0.2, 1.0]),
        (imgui::StyleColor::WindowBg, [0.05, 0.05, 0.05, 1.0]),
    };
}

pub struct UiManager {}

impl UiManager {

    pub fn draw(renderer: &mut Renderer, frame: &mut glium::Frame, ui: &mut imgui::Ui) {
        use imgui::*;
        use glium::Surface;

        let style = ui.push_style_colors(UI_STYLE.iter());
        let dimensions = frame.get_dimensions();

        // keeps track of the click states of the color picker
        let mut was_color_clicked = ClickState{ clicked: false, double_clicked: false };

        let color = unsafe {
            // *renderer.brush
            COLOR_PICKER_COLOR
        };

        // Left side-bar
        Window::new(im_str!("Textures"))
            .size([100.0, dimensions.1 as f32], Condition::Appearing)
            .size_constraints([100.0, dimensions.1 as f32], [dimensions.0 as f32, dimensions.1 as f32])
            .position([0.0, 0.0], Condition::Always)
            .bg_alpha(1.0)
            .movable(false)
            .collapsible(false)
            .build(ui, || {

                let button = imgui::ColorButton::new(im_str!("Custom Color"), color)
                    .size([40.0, 40.0])
                    .alpha(false);
                was_color_clicked.clicked = button.build(&ui);
                was_color_clicked.double_clicked = ui.is_item_hovered() && ui.is_mouse_double_clicked(MouseButton::Left);
                
                if was_color_clicked.clicked {
                    renderer.brush.set_pattern_brush(std::sync::Arc::new(PatternBrush{ color }));
                }

                if was_color_clicked.double_clicked {
                    unsafe {IS_COLOR_PICKER_OPEN = true;}
                }
            });
        
        // Color Picker window
        unsafe {
            if IS_COLOR_PICKER_OPEN {
                Window::new(im_str!("Color Picker"))
                    .opened(&mut IS_COLOR_PICKER_OPEN)
                    .always_auto_resize(true)
                    .collapsible(false)
                    .build(ui, || {
                        let picker = ColorPicker::new(im_str!(""), &mut COLOR_PICKER_COLOR)
                            .alpha(false);
                        if picker.build(&ui) {
                            renderer.brush.set_pattern_brush(std::sync::Arc::new(PatternBrush{ color: COLOR_PICKER_COLOR }));
                        }

                        if ui.button(im_str!("Close"), [200.0, 20.0]) {
                            IS_COLOR_PICKER_OPEN = false;
                        }
                    });
                
            }
        }

        Window::new(im_str!("Performance"))
            .always_auto_resize(true)
            .collapsible(true)
            .position([100.0, 0.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("{}ms", renderer.frame_timing.delta_frame_time().num_milliseconds()));
                ui.text(im_str!("{:.0} fps", 1.0 / (renderer.frame_timing.delta_frame_time().num_microseconds().unwrap() as f64 / 1_000_000.0)));
                ui.text(im_str!("{} draws", renderer.quilt.draw_stats.draws));
                ui.text(im_str!("{} vertices", renderer.quilt.draw_stats.vertices));
                ui.text(im_str!("{} indices", renderer.quilt.draw_stats.indices));
            });
        
        // Right side-bar
        Window::new(im_str!("Block Designs"))
            .size([100.0, dimensions.1 as f32], Condition::Appearing)
            .size_constraints([100.0, dimensions.1 as f32], [dimensions.0 as f32, dimensions.1 as f32])
            .position([dimensions.0 as f32, 0.0], Condition::Always)
            .position_pivot([1.0, 0.0])
            .bg_alpha(1.0)
            .movable(false)
            .collapsible(false)
            .build(ui, || {

                let block_list = crate::quilt::block::block_manager::BLOCK_LIST.lock().unwrap();

                for block_pattern in block_list.iter() {
                    if block_pattern.get_texture_id().is_some() && ImageButton::new(block_pattern.get_texture_id().unwrap(), [64.0, 64.0]).frame_padding(0).build(&ui) {
                        renderer.brush.set_block_brush(std::sync::Arc::new(BlockBrush {square_pattern: block_pattern.clone()}));
                    }
                    if ui.is_item_hovered() {
                        ui.tooltip(|| {
                            ui.text(block_pattern.get_pattern_name());
                            ImageButton::new(block_pattern.get_texture_id().unwrap(), [128.0, 128.0]).build(&ui);
                        });
                    }
                }
            });

        
        // Checks if any imgui window was clicked, if not tell the renderer that it was clicked
        if ui.is_mouse_clicked(MouseButton::Left) {
            if !ui.is_window_hovered_with_flags(WindowHoveredFlags::all()) {
                renderer.clicked();
            }
        }

        style.pop(&ui);
    }
}
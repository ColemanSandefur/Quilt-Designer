use crate::program::Program;
use crate::program::quilt::brush::*;
use crate::renderer::anti_aliasing::AntiAliasMode;

use lazy_static::lazy_static;
use parking_lot::Mutex;
use imgui::StyleVar;

struct ClickState {
    pub clicked: bool,
    pub double_clicked: bool,
}

static mut IS_COLOR_PICKER_OPEN: bool = false;
static mut COLOR_PICKER_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

lazy_static! {
    pub static ref UI_STYLE_COLOR: Vec<(imgui::StyleColor, [f32; 4])> = vec! {
        (imgui::StyleColor::ResizeGrip, [0.0; 4]),
        (imgui::StyleColor::ResizeGripActive, [0.0; 4]),
        (imgui::StyleColor::ResizeGripHovered, [0.0; 4]),
        (imgui::StyleColor::Text, [1.0, 1.0, 1.0, 1.0]),
        (imgui::StyleColor::TitleBg, [0.2, 0.2, 0.2, 1.0]),
        (imgui::StyleColor::TitleBgActive, [0.2, 0.2, 0.2, 1.0]),
        (imgui::StyleColor::MenuBarBg, [0.15, 0.15, 0.15, 1.0]),
        (imgui::StyleColor::WindowBg, [0.05, 0.05, 0.05, 1.0]),
    };

    pub static ref UI_STYLE_VAR: Vec<imgui::StyleVar> = vec! {
        StyleVar::WindowPadding([5.0, 5.0]),
        StyleVar::ItemSpacing([5.0, 5.0]),
        StyleVar::WindowBorderSize(0.0)
    };
}

pub struct UiManager {}

impl UiManager {

    const BUTTON_SIZE: f32 = 64.0;

    // returns if screen was clicked and an imgui window wasn't clicked
    pub fn draw(program: &mut Program, frame: &mut impl glium::Surface, ui: &mut imgui::Ui) -> bool {
        use imgui::*;

        let style_colors = ui.push_style_colors(UI_STYLE_COLOR.iter());
        let style_vars = ui.push_style_vars(UI_STYLE_VAR.iter());
        let dimensions = frame.get_dimensions();
        let current_style = ui.clone_style();
        
        // keeps track of the click states of the color picker
        let mut was_color_clicked = ClickState{ clicked: false, double_clicked: false };
        let color = unsafe {COLOR_PICKER_COLOR};

        let mut main_menu_bar_size = [0.0; 2];

        let style = ui.push_style_var(StyleVar::WindowBorderSize(1.0));
        ui.main_menu_bar(|| {
            ui.menu(im_str!("File"), true, || {
                if ui.small_button(im_str!("New")) {
                    SetupUi::open_window();
                }
                
                if ui.small_button(im_str!("Open")) {
                    program.load_quilt();
                }
                
                if ui.small_button(im_str!("Save")) {
                    program.save_quilt();
                }
            });

            ui.menu(im_str!("Window"), true, || {
                ui.menu(im_str!("Anti-Aliasing"), true, || {
                    if ui.small_button(im_str!("None")) {
                        program.get_renderer_mut().get_anti_aliasing_mut().set_multi_sample_mode(AntiAliasMode::NONE);
                    }

                    if ui.small_button(im_str!("FXAA")) {
                        // program.get_renderer_mut().fxaa_enabled = !program.get_renderer_mut().fxaa_enabled;
                        program.get_renderer_mut().get_anti_aliasing_mut().set_multi_sample_mode(AntiAliasMode::FXAA);
                    }

                    ui.menu(im_str!("MSAA"), true, || {
                        // create buttons for 2x, 4x, 8x, 16x MSAA
                        for i in 1..5 {
                            let sample = 2u32.pow(i);
                            if ui.small_button(&ImString::new(format!("{}x", sample))) {
                                program.get_renderer_mut().get_anti_aliasing_mut().set_multi_sample_mode(AntiAliasMode::MSAA(sample));
                            }
                        }
                    });
                })
            });

            main_menu_bar_size = ui.window_size();
        });
        style.pop(ui);
            
        // Left side-bar
        Window::new(im_str!("Textures"))
            .size([100.0, dimensions.1 as f32 - main_menu_bar_size[1]], Condition::Appearing)
            .size_constraints([100.0, dimensions.1 as f32 - main_menu_bar_size[1]], [dimensions.0 as f32, dimensions.1 as f32 - main_menu_bar_size[1]])
            .position([0.0, main_menu_bar_size[1]], Condition::Always)
            .bg_alpha(1.0)
            .movable(false)
            .collapsible(false)
            .build(ui, || {
                // calculates how many columns can fit in the window
                let num_buttons = 1 + crate::renderer::textures::get_texture_count() as i32; // will be the total of all textures (once they are added)
                let num_columns = std::cmp::max(1, std::cmp::min(((ui.window_content_region_width() - current_style.window_padding[0]) / (Self::BUTTON_SIZE + 2.0 * current_style.window_padding[0])) as i32, num_buttons));
                ui.columns(num_columns, im_str!("columns"), false);

                let offset = if ui.column_count() > 1 {current_style.window_padding[0]} else {0.0};
                let indentation = ui.current_column_width() / 2.0 - Self::BUTTON_SIZE / 2.0 - offset;

                // create color picker button
                ui.indent_by(indentation);

                let button = imgui::ColorButton::new(im_str!("Custom Color"), color)
                    .size([Self::BUTTON_SIZE, Self::BUTTON_SIZE])
                    .alpha(false);
                was_color_clicked.clicked = button.build(&ui);
                was_color_clicked.double_clicked = ui.is_item_hovered() && ui.is_mouse_double_clicked(MouseButton::Left);
                
                if was_color_clicked.clicked {
                    program.get_brush_mut().lock().set_pattern_brush(std::sync::Arc::new(PatternBrush::new_color(color)));
                }

                if was_color_clicked.double_clicked {
                    unsafe {IS_COLOR_PICKER_OPEN = true;}
                }

                ui.unindent_by(indentation);
                ui.next_column();

                for id in crate::renderer::textures::get_textures() {
                    // create texture button
                    ui.indent_by(indentation);
                    if ImageButton::new(id.get_imgui_id(), [Self::BUTTON_SIZE, Self::BUTTON_SIZE]).frame_padding(0).uv0([0.0, 1.0]).uv1([1.0, 0.0]).build(&ui) {

                        // on button click

                        // change brush to apply texture on click
                        program.get_brush_mut().lock().set_pattern_brush(std::sync::Arc::new(crate::program::quilt::brush::PatternBrush::new_texture(id.clone())));
                    }
                    ui.unindent_by(indentation);
                    ui.next_column();

                    // tooltip setup
                    if ui.is_item_hovered() {
                        ui.tooltip(|| {
                            Image::new(id.get_imgui_id(), [128.0, 128.0]).uv0([0.0, 1.0]).uv1([1.0, 0.0]).build(&ui);
                        });
                    }
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
                            program.get_brush_mut().lock().set_pattern_brush(std::sync::Arc::new(PatternBrush::new_color(COLOR_PICKER_COLOR)));
                        }

                        if ui.button(im_str!("Close"), [ui.window_content_region_width(), 20.0]) {
                            IS_COLOR_PICKER_OPEN = false;
                        }
                    });
                
            }
        }

        Window::new(im_str!("Performance"))
            .always_auto_resize(true)
            .collapsible(true)
            .position([100.0, main_menu_bar_size[1]], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("{}ms", program.get_renderer_mut().frame_timing.delta_frame_time().num_microseconds().unwrap() as f64 / 1000.0));
                ui.text(im_str!("{:.0} fps", 1.0 / (program.get_renderer_mut().frame_timing.delta_frame_time().num_microseconds().unwrap() as f64 / 1_000_000.0)));
                ui.text(im_str!("{} vertices", program.get_renderer_mut().get_vertex_count()));
                ui.text(im_str!("{} indices", program.get_renderer_mut().get_index_count()));
                ui.text(im_str!("{} render entries", program.get_renderer_mut().get_num_entries()));
                ui.text(im_str!("{} picker entries", program.get_renderer_mut().get_picker_mut().get_table().lock().num_keys()));
            });
        
        // Right side-bar
        Window::new(im_str!("Block Designs"))
            .size([100.0, dimensions.1 as f32 - main_menu_bar_size[1]], Condition::Appearing)
            .size_constraints([100.0, dimensions.1 as f32 - main_menu_bar_size[1]], [dimensions.0 as f32, dimensions.1 as f32 - main_menu_bar_size[1]])
            .position([dimensions.0 as f32, main_menu_bar_size[1]], Condition::Always)
            .position_pivot([1.0, 0.0])
            .bg_alpha(1.0)
            .movable(false)
            .collapsible(false)
            .build(ui, || {
                let block_list = crate::program::quilt::block::block_manager::BLOCK_LIST.lock().unwrap();

                // calculates how many columns can fit in the window
                let num_buttons = block_list.len() as i32;
                let num_columns = std::cmp::max(1, std::cmp::min(((ui.window_content_region_width() - current_style.window_padding[0]) / (Self::BUTTON_SIZE + 2.0 * current_style.window_padding[0])) as i32, num_buttons));
                ui.columns(num_columns, im_str!("columns"), false);

                let offset = if ui.column_count() > 1 {current_style.window_padding[0]} else {0.0};
                let indentation = ui.current_column_width() / 2.0 - Self::BUTTON_SIZE / 2.0 - offset;

                for block_pattern in block_list.iter() {

                    // create texture button
                    ui.indent_by(indentation);
                    if block_pattern.get_texture_id().is_some() && ImageButton::new(block_pattern.get_texture_id().unwrap(), [Self::BUTTON_SIZE, Self::BUTTON_SIZE]).frame_padding(0).build(&ui) {

                        // on button click

                        program.get_brush_mut().lock().set_block_brush(std::sync::Arc::new(BlockBrush {square_pattern: block_pattern.clone()}));
                    }
                    ui.unindent_by(indentation);
                    ui.next_column();

                    // tooltip setup
                    if ui.is_item_hovered() {
                        ui.tooltip(|| {
                            ui.text_wrapped(&ImString::from(block_pattern.get_pattern_name().clone()));
                            ui.separator();
                            Image::new(block_pattern.get_texture_id().unwrap(), [128.0, 128.0]).build(&ui);
                        });
                    }

                }
            });

        SetupUi::draw(program, frame, ui);
        
            
        style_colors.pop(&ui);
        style_vars.pop(&ui);
        
        // Checks if any imgui window was clicked, if not tell the renderer that it was clicked
        if ui.is_mouse_clicked(MouseButton::Left) {
            if !ui.is_window_hovered_with_flags(WindowHoveredFlags::all()) {
                return true;
            }
        }

        false
    }
}

//
// QuiltUi
//
// The main gui (side-bars for selecting color and patterns, etc)
//

// mod quilt_ui {
//     pub struct QuiltUI {

//     }
// }

//
// SetupUi
//
// Used for prompting the user to create a quilt
//

pub use setup_ui::*; // Re-export for convenience 
mod setup_ui {
    use super::*;
    use imgui::*;

    lazy_static!  {
        static ref QUILT_DIMENSIONS: Mutex<(i32, i32)> = Mutex::new((SetupUi::DEFAULT_QUILT_DIMENSIONS.0 as i32, SetupUi::DEFAULT_QUILT_DIMENSIONS.1 as i32));
        static ref QUILT_PROMPT_OPEN: Mutex<bool> = Mutex::new(false);
    }

    pub struct SetupUi {}

    impl SetupUi {
        const CREATION_BUTTON_DIMENSIONS: [f32; 2] = [100.0, 20.0];
        const DEFAULT_QUILT_DIMENSIONS: (u32, u32) = (6, 8);
        const WARNING_SIZE: u32 = 50; // Display a warning when a quilt dimensions gets larger than this (potential performance issues)

        pub fn open_window() {
            *QUILT_PROMPT_OPEN.lock() = true;
        }

        pub fn close_window() {
            *QUILT_PROMPT_OPEN.lock() = false;
        }

        fn reset_dimensions() {
            *QUILT_DIMENSIONS.lock() = (SetupUi::DEFAULT_QUILT_DIMENSIONS.0 as i32, SetupUi::DEFAULT_QUILT_DIMENSIONS.1 as i32)
        }

        pub fn draw(program: &mut Program, frame: &mut impl glium::Surface, ui: &mut imgui::Ui) -> bool {
            let dimensions = frame.get_dimensions();

            let style_var = ui.push_style_vars(&vec![
                StyleVar::WindowPadding([20.0; 2]),
                StyleVar::WindowRounding(10.0),
            ]);

            let style_colors = ui.push_style_colors(&vec![
                (StyleColor::Button, [0.3, 0.3, 0.3, 1.0]),
            ]);
            
            let mut quilt_prompt_open = QUILT_PROMPT_OPEN.lock();
            
            // Automatically set quilt prompt to open when program doesn't have a quilt yet
            *quilt_prompt_open = *quilt_prompt_open || !program.has_quilt();
            
            if *quilt_prompt_open {
                let mut cancel_clicked = false;
                let mut create_clicked = false;

                Window::new(im_str!("New Quilt"))
                    .always_auto_resize(true)
                    .opened(&mut *quilt_prompt_open)
                    .collapsible(false)
                    .position_pivot([0.5; 2])
                    .position([dimensions.0 as f32 / 2.0, dimensions.1 as f32 / 2.0], Condition::Always)
                    .movable(false)
                    .resizable(false)
                    .build(ui, || {
                        let mut quilt_dimensions = QUILT_DIMENSIONS.lock();
                        ui.input_int(im_str!("Width"), &mut quilt_dimensions.0).build();
                        ui.input_int(im_str!("Height"), &mut quilt_dimensions.1).build();

                        // Make sure that neither of the dimensions ever go below 1
                        quilt_dimensions.0 = std::cmp::max(1, quilt_dimensions.0);
                        quilt_dimensions.1 = std::cmp::max(1, quilt_dimensions.1);
                        
                        // Display a performance warning if quilt is becoming too large
                        if quilt_dimensions.0 > Self::WARNING_SIZE as i32 || quilt_dimensions.1 > Self::WARNING_SIZE as i32 {
                            ui.text_wrapped(im_str!("Warning: large quilts can have performance issues"));
                        } else {
                            ui.new_line();
                        }

                        if ui.button(im_str!("Cancel"), Self::CREATION_BUTTON_DIMENSIONS) {
                            cancel_clicked = true;
                        }
                        ui.same_line(0.0);
                        if ui.button(im_str!("Create"), Self::CREATION_BUTTON_DIMENSIONS) {
                            create_clicked = true;
                        }
                    });
                
                if cancel_clicked {
                    *quilt_prompt_open = false;
                }

                if create_clicked {
                    *quilt_prompt_open = false;
                    
                    let quilt_dimensions = QUILT_DIMENSIONS.lock();
                    program.new_quilt(quilt_dimensions.0 as usize, quilt_dimensions.1 as  usize);
                }

                // reset the dimensions if the quilt creation window was closed this frame
                if !*quilt_prompt_open {
                    Self::reset_dimensions();
                }
            }


            style_var.pop(&ui);
            style_colors.pop(&ui);

            false
        }
    }
}
pub mod quilt;
pub mod ui_manager;

use crate::parse::{SaveData};
use crate::renderer::Renderer;
use crate::renderer::util::keyboard_tracker::KeyboardTracker;
use ui_manager::UiManager;
use quilt::Quilt;
use quilt::brush::{Brush, PatternBrush};

use std::rc::Rc;
use glium::glutin::event::*;
use std::io::Write;

#[allow(dead_code)]
pub struct Program {
    display: Rc<glium::Display>,
    keyboard_tracker: KeyboardTracker,
    renderer: Renderer,
    quilt: Quilt,
    brush: Brush
}

impl Program {
    pub fn new(display: Rc<glium::Display>) -> Self {
        let mut renderer = Renderer::new(display.clone());
        let quilt = Quilt::new(6, 8, renderer.get_picker_mut());

        let dimensions = quilt.get_dimensions();
        renderer.get_world_transform_mut().set_scale(1.0, 1.0, std::cmp::max(dimensions.0, dimensions.1) as f32 * 1.0);

        Self {
            display: display.clone(),
            renderer,
            quilt,
            keyboard_tracker: KeyboardTracker::new(),
            brush: Brush::new_pattern_brush(PatternBrush::new_color([1.0;4])),
        }
    }

    pub fn draw(&mut self, frame: &mut glium::Frame, ui: &mut imgui::Ui) {
        self.quilt.draw(&mut self.renderer);

        self.renderer.start_frame();
        self.renderer.render(frame);

        if UiManager::draw(self, frame, ui) {self.handle_click()};

        self.handle_keys();

        self.renderer.end_frame();
    }

    pub fn window_event(&mut self, event: &WindowEvent) {
        // println!("{:?}", event);

        if let WindowEvent::KeyboardInput{input, ..} = event {
            self.key_pressed_event(&input);

            if let Some(keycode) = input.virtual_keycode {
                self.keyboard_tracker.set_pressed(keycode, input.state == ElementState::Pressed);
            }
        }

        if let WindowEvent::CursorMoved{position, ..} = event {
            self.renderer.cursor_moved(position);
        }

        if let WindowEvent::Focused(is_focused) = event {
            if !is_focused {
                self.keyboard_tracker.release_all();
            }
        }
    }

    fn key_pressed_event(&mut self, event: &KeyboardInput) {
        if let Some(virtual_keycode) = event.virtual_keycode {

            if event.state == ElementState::Pressed {
                match virtual_keycode {
                    VirtualKeyCode::R => {
                        if self.keyboard_tracker.is_shift_pressed() {
                            Brush::increase_rotation(std::f32::consts::FRAC_PI_2);
                        } else {
                            Brush::increase_rotation(-std::f32::consts::FRAC_PI_2);
                        }
                    },

                    VirtualKeyCode::T => {
                        self.save_quilt("test.quilt");
                    }

                    VirtualKeyCode::U => {
                        self.quilt = Quilt::new(1, 1, self.renderer.get_picker_mut());
                    }
                    _ => ()
                }
            }

        }
    }

    fn handle_keys(&mut self) {
        let keyboard_tracker = &mut self.keyboard_tracker;

        let delta_time = self.renderer.frame_timing.delta_frame_time().num_microseconds().unwrap() as f32 / 1_000.0;
        let movement_speed = 0.003;

        if keyboard_tracker.is_key_pressed(&VirtualKeyCode::A) {
            self.renderer.get_world_transform_mut().translate(delta_time * movement_speed, 0.0, 0.0);
        }
        if keyboard_tracker.is_key_pressed(&VirtualKeyCode::D) {
            self.renderer.get_world_transform_mut().translate(delta_time * -movement_speed, 0.0, 0.0);
        }
        if keyboard_tracker.is_key_pressed(&VirtualKeyCode::W) {
            self.renderer.get_world_transform_mut().translate(0.0, delta_time * -movement_speed, 0.0);
        }
        if keyboard_tracker.is_key_pressed(&VirtualKeyCode::S) {
            self.renderer.get_world_transform_mut().translate(0.0, delta_time * movement_speed, 0.0);
        }

        let zoom_speed = 0.005;
        let zoom_threshold = -0.7;

        if keyboard_tracker.is_key_pressed(&VirtualKeyCode::Q) {
            self.renderer.get_world_transform_mut().add_scale(0.0, 0.0, delta_time * zoom_speed);
        }
        if keyboard_tracker.is_key_pressed(&VirtualKeyCode::E) {
            self.renderer.get_world_transform_mut().add_scale(0.0, 0.0, delta_time * -zoom_speed);

            let translation = self.renderer.get_world_transform_mut().get_scale();
            if translation.2 <= zoom_threshold {
                self.renderer.get_world_transform_mut().set_scale(translation.0, translation.1, zoom_threshold);
            }
        }
    }

    fn handle_click(&mut self) {

        if let Some(picker_entry) = self.renderer.clicked() {
            let picker_entry = picker_entry.clone();
            self.quilt.click(&picker_entry, &self.brush, &mut self.renderer.get_picker_mut());
        }
    }

    pub fn get_renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    pub fn get_brush_mut(&mut self) -> &mut Brush {
        &mut self.brush
    }

    fn save_quilt(&self, name: &str) {
        let path_name = format!("./saves/{}", name);
        let path = std::path::Path::new(&path_name);
        let file = std::fs::File::create(path).unwrap();
        let zip = zip::ZipWriter::new(file);

        let mut save_data = SaveData {
            writer: Some(zip),
            reader: None,
            files_written: Vec::new(),
        };

        println!("Started saving");

        let yaml = self.quilt.to_save(&mut save_data);

        let output = yaml.dump_to_string();
        let mut zip = save_data.writer.unwrap();

        zip.start_file("save.yaml", Default::default()).unwrap();
        write!(zip, "{}", output).unwrap();

        zip.finish().unwrap();

        println!("Finished saving");
    }
}
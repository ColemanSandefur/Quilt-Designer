pub mod quilt;
pub mod ui_manager;

use crate::renderer::Renderer;
use ui_manager::UiManager;
use quilt::Quilt;
use quilt::brush::{Brush, PatternBrush};

use std::rc::Rc;
use glium::glutin::event::WindowEvent;

pub struct Program {
    display: Rc<glium::Display>,
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
            brush: Brush::new_pattern_brush(PatternBrush::new_color([1.0;4])),
        }
    }

    pub fn draw(&mut self, frame: &mut glium::Frame, ui: &mut imgui::Ui) {
        self.quilt.draw(&mut self.renderer);

        self.renderer.start_frame();
        self.renderer.render(frame);

        if UiManager::draw(self, frame, ui) {self.handle_click()};

        self.renderer.end_frame();
    }

    pub fn window_event(&mut self, event: &WindowEvent) {
        // println!("{:?}", event);

        if let WindowEvent::CursorMoved{position, ..} = &event {
            self.renderer.cursor_moved(position);
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
}
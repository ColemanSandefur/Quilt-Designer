use crate::quilt::Quilt;
use crate::util::frame_timing::FrameTiming;
use crate::util::keyboard_tracker::KeyboardTracker;
use crate::render::material::material_manager::MaterialManager;
use crate::render::matrix::{Matrix, WorldTransform};
use crate::render::ui_manager::UiManager;
use crate::render::picker::Picker;

use glium::Surface;

#[allow(dead_code)]
pub struct Renderer {
    pub shaders: MaterialManager,
    pub world_transform: Matrix,
    pub frame_timing: FrameTiming,
    pub keyboard_tracker: KeyboardTracker,
    pub quilt: Quilt,
    pub picker: Picker,
    pub cursor_pos: Option<(i32, i32)>,
}

impl Renderer {
    pub fn new(display: &dyn glium::backend::Facade) -> Self {
        let mut shaders = MaterialManager::load_all(display);

        let mut world_transform = Matrix::new_with_data([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0, 1.0],
        ]);

        
        let mut picker = Picker::new(display, &shaders);
        let quilt = Quilt::new(display, &mut shaders, 6 * 4, 8 * 4, &mut picker);

        //garbage way to fit quilt to screen
        let dimensions = quilt.get_dimensions();
        world_transform.set_scale(1.0, 1.0, std::cmp::max(dimensions.0, dimensions.1) as f32 * 1.0);


        Self {
            shaders,
            world_transform,
            frame_timing: FrameTiming::new(),
            keyboard_tracker: KeyboardTracker::new(),
            quilt,
            picker,
            cursor_pos: None,
            // picking_pixel_buffer: glium::texture::pixel_buffer::PixelBuffer::new_empty(display, 1),
        }
    }

    pub fn draw(&mut self, target: &mut glium::Frame, ui: &mut imgui::Ui) {

        target.clear_color(0.02, 0.02, 0.02, 1.0);

        let projection = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            Matrix::new_with_data([
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ])
        };

        let global_transform = WorldTransform {
            projection: projection,
            world: self.world_transform,
        };

        self.quilt.draw(target, &global_transform, &Default::default(), &mut self.picker);

        UiManager::draw(self, target, ui);
        self.handle_keys();

        self.frame_timing.update_frame_time();
    }

    fn handle_keys(&mut self) {
        use glium::glutin::event::VirtualKeyCode;

        let keyboard_tracker = &mut self.keyboard_tracker;

        let delta_time = self.frame_timing.delta_frame_time().num_microseconds().unwrap() as f32 / 1_000.0;
        let movement_speed = 0.003;

        if keyboard_tracker.is_key_pressed(&VirtualKeyCode::A) {
            self.world_transform.translate(delta_time * movement_speed, 0.0, 0.0);
        }
        if keyboard_tracker.is_key_pressed(&VirtualKeyCode::D) {
            self.world_transform.translate(delta_time * -movement_speed, 0.0, 0.0);
        }
        if keyboard_tracker.is_key_pressed(&VirtualKeyCode::W) {
            self.world_transform.translate(0.0, delta_time * -movement_speed, 0.0);
        }
        if keyboard_tracker.is_key_pressed(&VirtualKeyCode::S) {
            self.world_transform.translate(0.0, delta_time * movement_speed, 0.0);
        }

        let zoom_speed = 0.005;
        let zoom_threshold = -0.7;

        if keyboard_tracker.is_key_pressed(&VirtualKeyCode::Q) {
            self.world_transform.add_scale(0.0, 0.0, delta_time * zoom_speed);
        }
        if keyboard_tracker.is_key_pressed(&VirtualKeyCode::E) {
            self.world_transform.add_scale(0.0, 0.0, delta_time * -zoom_speed);

            let translation = self.world_transform.get_scale();
            if translation.2 <= zoom_threshold {
                self.world_transform.set_scale(translation.0, translation.1, zoom_threshold);
            }
        }
    }

    pub fn clicked(&mut self) {
        if let Some(cursor) = self.cursor_pos {
            self.picker.click(cursor);

            let entry = self.picker.get_clicked();

            
            if let Some(entry) = entry {
                self.quilt.click(&entry);
            }
        }
    }

    pub fn cursor_moved(&mut self, position: &glium::glutin::dpi::PhysicalPosition<f64>) {
        self.cursor_pos = Some(position.cast::<i32>().into());
    }
}
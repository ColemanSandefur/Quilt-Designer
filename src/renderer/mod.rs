pub mod util;
pub mod material;
pub mod matrix;
pub mod shape;
pub mod picker;
pub mod textures;
pub mod vertex;
pub mod shape_object;

use picker::{Picker, PickerEntry};
use vertex::Vertex;
use matrix::{Matrix, WorldTransform};
use util::frame_timing::FrameTiming;

use std::collections::HashMap;
use rand::prelude::*;
use std::rc::Rc;
use glium::{VertexBuffer, IndexBuffer};
use glium::Surface;

pub struct Renderer {
    world_transform: Matrix,
    random_gen: ThreadRng,
    display: Rc<glium::Display>,
    picker: Picker,
    pub frame_timing: FrameTiming,
    pub cursor_pos: Option<(i32, i32)>,
    
    // Holds all items that will be rendered
    render_items: HashMap<u32, Vec<Box<dyn Renderable>>>,

    buffers_need_updated: bool,
    vertex_vec: Vec<Vertex>,
    index_vec: Vec<u32>,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
    vertex_len: usize,
    index_len: usize,
}

impl Renderer {
    // Initial sizes of the vertex and index buffers
    pub const INIT_VERTICES: usize = 6000;
    pub const INIT_INDICES: usize = Self::INIT_VERTICES * 4;

    pub fn new(display: Rc<glium::Display>) -> Self {
        let world_transform = Matrix::new_with_data([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0, 1.0],
        ]);

        Self {
            world_transform,
            render_items: HashMap::with_capacity(10),
            random_gen: rand::thread_rng(),
            buffers_need_updated: false,
            display: display.clone(),
            picker: Picker::new(&*display),
            frame_timing: FrameTiming::new(),
            cursor_pos: None,

            vertex_buffer: VertexBuffer::empty_dynamic(&*display, Self::INIT_VERTICES).unwrap(),
            index_buffer: IndexBuffer::empty_dynamic(&*display, glium::index::PrimitiveType::TrianglesList, Self::INIT_INDICES).unwrap(),
            vertex_vec: Vec::with_capacity(Self::INIT_VERTICES),
            index_vec: Vec::with_capacity(Self::INIT_INDICES),
            vertex_len: 0,
            index_len:0,
        }
    }

    // Subscribes items to be rendered
    pub fn add_render_items(&mut self, render_items: Vec<Box<dyn Renderable>>) -> RenderToken {
        let id = self.get_new_id();

        for item in &render_items {
            self.vertex_len += item.get_vertex_count();
            self.index_len += item.get_index_count();
        }
        self.render_items.insert(id, render_items);

        self.buffers_need_updated = true;

        id.into()
    }

    // Modify an existing render subscription
    pub fn set_render_items(&mut self, render_items: Vec<Box<dyn Renderable>>, render_id: RenderToken) {
        for item in &render_items {
            self.vertex_len += item.get_vertex_count();
            self.index_len += item.get_index_count();
        }

        let old_render_items = self.render_items.insert(render_id.into(), render_items);

        if let Some(render_items) = old_render_items {
            for item in &render_items {
                self.vertex_len -= item.get_vertex_count();
                self.index_len -= item.get_index_count();
            }
        }

        self.buffers_need_updated = true;
    }

    // Remove a subscription
    pub fn remove_id(&mut self, token: RenderToken) {
        let old_render_items = self.render_items.remove(&token.into());

        if let Some(render_items) = old_render_items {
            for item in &render_items {
                self.vertex_len -= item.get_vertex_count();
                self.index_len -= item.get_index_count();
            }
        }

        self.buffers_need_updated = true;
    }

    pub fn get_new_id(&mut self) -> u32{
        let mut num: u32 = self.random_gen.gen();

        while self.render_items.contains_key(&num) {
            num = self.random_gen.gen()
        }

        num
    }

    // When a render subscription has changed we should rebuild the buffers
    pub fn rebuild_buffers(&mut self) {
        // check if we should resize the buffers

        if self.vertex_len > self.vertex_buffer.len() {
            self.vertex_buffer = VertexBuffer::empty_dynamic(&*self.display, (self.vertex_len as f32 * 1.1) as usize).unwrap();
            self.vertex_vec = Vec::with_capacity((self.vertex_len as f32 * 1.1) as usize);
        }

        if self.index_len > self.index_buffer.len() {
            self.index_buffer = IndexBuffer::empty_dynamic(&*self.display, glium::index::PrimitiveType::TrianglesList, (self.index_len as f32 * 1.1) as usize).unwrap();
            self.index_vec = Vec::with_capacity((self.index_len as f32 * 1.1) as usize);
        }

        // Fill the buffers

        self.index_vec.clear();
        self.vertex_vec.clear();

        for val in self.render_items.values() {
            for item in val {
                item.add_to_ib_vec(&mut self.index_vec, self.vertex_vec.len());
                item.add_to_vb_vec(&mut self.vertex_vec);
            }
        }

        self.vertex_buffer.invalidate();
        self.index_buffer.invalidate();

        self.vertex_buffer.slice_mut(0..self.vertex_vec.len()).expect("Invalid vertex range").write(&self.vertex_vec);
        self.index_buffer.slice_mut(0..self.index_vec.len()).expect("Invalid index range").write(&self.index_vec); 

        // Really bad way to invalidate index buffer, calling invalidate doesn't seem to do anything, just writes 0s to the rest of the buffer
        if self.index_buffer.len() - self.index_vec.len() > 0 {
            let slice = self.index_buffer.slice(self.index_vec.len()..).expect("Invalid index range");
            let buffer: Vec<u32> = vec![0; slice.len()];
            slice.write(&buffer);
        }
    }

    pub fn start_frame(&mut self) {
        // put any code that should be ran at the beginning of a frame here
    }

    pub fn end_frame(&mut self) {
        // any code that runs at the end of a frame
        self.frame_timing.update_frame_time();
    }

    pub fn render(&mut self, target: &mut glium::Frame) {
        if self.buffers_need_updated {
            self.rebuild_buffers();
            self.buffers_need_updated = false;
        }

        target.clear_color(0.02, 0.02, 0.02, 1.0);
        self.picker.clear_surface(target, &*self.display);

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

        material::get_material_manager().get_solid_color_material().draw(&(&self.vertex_buffer, &self.index_buffer), target, &global_transform, &Default::default());

        self.picker.draw(self.vertex_buffer.get_context(), &global_transform, &self.vertex_buffer, &self.index_buffer, &Default::default());
    }


    pub fn get_picker_mut(&mut self) -> &mut Picker {
        &mut self.picker
    }

    pub fn get_world_transform_mut(&mut self) -> &mut Matrix {
        &mut self.world_transform
    }

    pub fn clicked(&mut self) -> Option<&PickerEntry>{
        if let Some(cursor) = self.cursor_pos {
            self.picker.click(cursor);

            return self.picker.get_clicked();
        }

        None
    }

    pub fn cursor_moved(&mut self, position: &glium::glutin::dpi::PhysicalPosition<f64>) {
        self.cursor_pos = Some(position.cast::<i32>().into());
    }

    pub fn get_index_count(&self) -> usize {
        self.index_len
    }

    pub fn get_vertex_count(&self) -> usize {
        self.vertex_len
    }
}

pub trait Renderable {
    fn get_index_count(&self) -> usize;

    fn get_vertex_count(&self) -> usize;

    fn get_vb(&self) -> &Vec<Vertex>;

    fn get_ib(&self) -> &Vec<u32>;

    fn add_to_vb_vec(&self, vertex_buffer: &mut Vec<Vertex>) {
        let vb = self.get_vb();
        for i in 0..self.get_vertex_count() {
            vertex_buffer.push(vb[i]);
        }
    }

    fn add_to_ib_vec(&self, index_buffer: &mut Vec<u32>, vb_index: usize) {
        let ib = self.get_ib();
        for i in 0..self.get_index_count() {
            index_buffer.push(vb_index as u32 + ib[i]);
        }
    }

    fn can_fit_in_buffers(&self, vb_capacity: usize, ib_capacity: usize, vb_index: usize, ib_index: usize) -> bool {
        vb_index + self.get_vertex_count() <= vb_capacity && ib_index + self.get_index_count() <= ib_capacity
    }
}

#[must_use]
#[derive(Clone, Copy)]
pub struct RenderToken(u32);

impl From<u32> for RenderToken {
    fn from(number: u32) -> Self {
        RenderToken(number)
    }
}

impl From<RenderToken> for u32 {
    fn from(token: RenderToken) -> Self {
        token.0
    }
}
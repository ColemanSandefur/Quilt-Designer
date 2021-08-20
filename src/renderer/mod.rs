pub mod util;
pub mod material;
pub mod matrix;
pub mod shape;
pub mod picker;
pub mod textures;
pub mod vertex;
pub mod shape_object;
pub mod new_picker;

use new_picker::{Picker};
use vertex::Vertex;
use matrix::{Matrix, WorldTransform};
use util::frame_timing::FrameTiming;

use std::collections::HashMap;
use rand::prelude::*;
use std::rc::{Weak, Rc};
use std::cell::RefCell;
use glium::{VertexBuffer, IndexBuffer};
use std::ops::Deref;

pub struct RenderTable {
    random_gen: ThreadRng,
    render_items: HashMap<u32, RenderItem>,
    was_updated: bool,
    vertex_len: usize,
    index_len: usize,

    self_rc: Option<Rc<RefCell<Self>>>,
}

impl RenderTable {
    pub fn new() -> Rc<RefCell<Self>> {
        let s = Self {
            render_items: HashMap::with_capacity(10),
            random_gen: rand::thread_rng(),
            was_updated: false,
            vertex_len: 0,
            index_len:0,
            self_rc: None,
        };

        let s = Rc::new(RefCell::new(s));

        s.borrow_mut().self_rc = Some(s.clone());

        s
    }

    // Subscribes items to be rendered
    // Tokens must be kept alive or else the item will be removed from the render queue
    pub fn add_render_items(&mut self, render_items: Vec<Box<dyn Renderable>>) -> RenderToken {
        let token = self.get_new_token();

        for item in &render_items {
            self.vertex_len += item.get_vertex_count();
            self.index_len += item.get_index_count();
        }

        let token: RenderToken = token;

        self.render_items.insert(*token.0, RenderItem {
            render_item: render_items,
        });

        self.was_updated = true;

        token
    }

    // Modify an existing render subscription
    pub fn set_render_items(&mut self, render_items: Vec<Box<dyn Renderable>>, render_id: RenderToken) {
        for item in &render_items {
            self.vertex_len += item.get_vertex_count();
            self.index_len += item.get_index_count();
        }

        let old_render_items = self.render_items.insert(*render_id.0, RenderItem {
            render_item: render_items,
        });

        if let Some(render_items) = old_render_items {
            for item in &render_items.render_item {
                self.vertex_len -= item.get_vertex_count();
                self.index_len -= item.get_index_count();
            }
        }

        self.was_updated = true;
    }

    // Remove a subscription
    pub fn remove_id(&mut self, token: RenderToken) {
        let old_render_items = self.render_items.remove(&token.0);

        if let Some(render_items) = old_render_items {
            for item in &render_items.render_item {
                self.vertex_len -= item.get_vertex_count();
                self.index_len -= item.get_index_count();
            }
        }

        self.was_updated = true;
    }

    pub fn get_new_token(&mut self) -> RenderToken {
        let mut num: u32 = self.random_gen.gen();

        while self.render_items.contains_key(&num) {
            num = self.random_gen.gen()
        }

        RenderToken::new(Rc::new(num), Rc::downgrade(&self.self_rc.as_ref().unwrap().clone()))
    }

    pub fn get_num_indices(&self) -> usize {
        self.index_len
    }

    pub fn get_num_vertices(&self) -> usize {
        self.vertex_len
    }

    pub fn needs_updated(&self) -> bool {
        self.was_updated
    }

    pub fn reset_needs_updated(&mut self) {
        self.was_updated = false;
    }

    fn iter(&self) -> std::collections::hash_map::Iter<'_, u32, RenderItem> {
        self.render_items.iter()
    }
}

pub struct Renderer {
    world_transform: Matrix,
    display: Rc<glium::Display>,
    picker: Picker,
    pub frame_timing: FrameTiming,
    pub cursor_pos: Option<(i32, i32)>,
    
    // Holds all items that will be rendered
    render_items: Rc<RefCell<RenderTable>>,

    vertex_vec: Vec<Vertex>,
    index_vec: Vec<u32>,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
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
            render_items: RenderTable::new(),
            display: display.clone(),
            picker: Picker::new(&*display),
            frame_timing: FrameTiming::new(),
            cursor_pos: None,

            vertex_buffer: VertexBuffer::empty_dynamic(&*display, Self::INIT_VERTICES).unwrap(),
            index_buffer: IndexBuffer::empty_dynamic(&*display, glium::index::PrimitiveType::TrianglesList, Self::INIT_INDICES).unwrap(),
            vertex_vec: Vec::with_capacity(Self::INIT_VERTICES),
            index_vec: Vec::with_capacity(Self::INIT_INDICES),
        }
    }

    // When a render subscription has changed we should rebuild the buffers
    pub fn rebuild_buffers(&mut self) {
        // check if we should resize the buffers

        if self.render_items.borrow().get_num_vertices() > self.vertex_buffer.len() {
            self.vertex_buffer = VertexBuffer::empty_dynamic(&*self.display, (self.render_items.borrow().get_num_vertices() as f32 * 1.1) as usize).unwrap();
            self.vertex_vec = Vec::with_capacity((self.render_items.borrow().get_num_vertices() as f32 * 1.1) as usize);
        }

        if self.render_items.borrow().get_num_indices() > self.index_buffer.len() {
            self.index_buffer = IndexBuffer::empty_dynamic(&*self.display, glium::index::PrimitiveType::TrianglesList, (self.render_items.borrow().get_num_indices() as f32 * 1.1) as usize).unwrap();
            self.index_vec = Vec::with_capacity((self.render_items.borrow().get_num_indices() as f32 * 1.1) as usize);
        }

        // Fill the buffers

        self.index_vec.clear();
        self.vertex_vec.clear();

        for (_, val) in self.render_items.borrow().iter() {
            for item in &val.render_item {
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

    pub fn render(&mut self, target: &mut impl glium::Surface) {
        if self.render_items.borrow().needs_updated() {
            self.rebuild_buffers();
            self.render_items.borrow_mut().reset_needs_updated();
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

    pub fn clicked(&mut self) {
        self.picker.click(self.cursor_pos.unwrap());
    }

    pub fn cursor_moved(&mut self, position: &glium::glutin::dpi::PhysicalPosition<f64>) {
        self.cursor_pos = Some(position.cast::<i32>().into());
    }

    pub fn get_index_count(&self) -> usize {
        self.render_items.borrow().get_num_indices()
    }

    pub fn get_vertex_count(&self) -> usize {
        self.render_items.borrow().get_num_vertices()
    }

    pub fn get_render_items_mut(&mut self) -> Rc<RefCell<RenderTable>> {
        self.render_items.clone()
    }

    pub fn get_num_entries(&self) -> usize {
        self.render_items.borrow().iter().len()
    }
}

pub trait Renderable {
    fn get_index_count(&self) -> usize;

    fn get_vertex_count(&self) -> usize;

    fn get_vb(&self) -> Vec<Vertex>;

    fn get_ib(&self) -> Vec<u32>;

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
#[derive(Clone)]
// Keeps a reference to both the id and render table, once the id has no more references we will remove it from the table
pub struct RenderToken(Rc<u32>, Weak<RefCell<RenderTable>>);

impl Deref for RenderToken {
    type Target = Rc<u32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RenderToken {
    pub fn new(id: Rc<u32>, render_table: Weak<RefCell<RenderTable>>) -> Self {
        RenderToken(id, render_table)
    }
}

impl Drop for RenderToken {
    fn drop(&mut self) {
        // this is the last token alive and we should remove the token from the table
        if Rc::strong_count(&self.0) <= 1 {
            if let Some(render_table) = Weak::upgrade(&self.1) {
                render_table.borrow_mut().remove_id(self.clone());
            }
        }
    }
}

struct RenderItem {
    render_item: Vec<Box<dyn Renderable>>,
}

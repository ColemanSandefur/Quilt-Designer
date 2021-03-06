use crate::renderer::vertex::Vertex;
use crate::renderer::material::{*};
use crate::renderer::matrix::{WorldTransform};
pub use picker_token::*;

use std::collections::{HashMap};
use glium::{VertexBuffer, IndexBuffer, Surface};
use std::sync::{Arc, Weak};
use parking_lot::Mutex;

// The type of function that will be called when clicked
pub type PickerTrait = dyn Fn(u32) + Send + Sync;

// Using mod to make sure that nothing has access to the picker fields of PickerToken and WeakPickerToken which will cause memory issues
pub mod picker_token {
    use super::*;

    //
    // PickerToken
    //
    // Given as a response to 'subscribing' to the Picker;
    // Once dropped it will remove itself from the PickerTable
    //

    #[must_use]
    #[derive(Clone)]
    pub struct PickerToken {
        picker: Arc<Mutex<PickerTrait>>, //You should never have access to the picker, can easily have leaks that way
        table: Weak<Mutex<PickerTable>>,
        pub id: u32,
    }
    
    impl Drop for PickerToken {
        fn drop(&mut self) {
            if Arc::strong_count(&self.picker) <= 1 {
                self.unsubscribe();
            }
        }
    }
    
    impl PickerToken {
        // automatically unsubscribes when all tokens have been dropped
        fn unsubscribe(&self) {
            if let Some(table) = Weak::upgrade(&self.table) {
                table.lock().table.remove(&self.id);
            }
        }

        pub fn run(&self) {
            self.picker.lock()(self.id);
        }

        pub fn new(picker: Arc<Mutex<PickerTrait>>, table: Weak<Mutex<PickerTable>>, id: u32) -> Self {
            Self {
                picker,
                table,
                id,
            }
        }
    }

    //
    // WeakPickerToken
    //
    // Used as a weak reference for PickerTokens
    // This prevents an issue of PickerToken's drop not activating because of handling a raw Arc<Mutex<PickerTrait>> and leaving an entry in the table that shouldn't exist
    // In order to access PickerTrait we must upgrade to a PickerToken so that PickerTrait will only be dropped along side PickerToken
    //
    
    #[derive(Clone)]
    pub struct WeakPickerToken {
        picker: Weak<Mutex<PickerTrait>>, //You should never have access to the picker, can easily have leaks
        table: Weak<Mutex<PickerTable>>,
        pub id: u32,
    }
    
    impl WeakPickerToken {
        pub fn upgrade(&self) -> Option<PickerToken> {
            if let Some(picker) = Weak::upgrade(&self.picker) {
                return Some(PickerToken {
                    picker,
                    table: self.table.clone(),
                    id: self.id,
                })
            }
    
            None
        }

        pub fn new(picker: Weak<Mutex<PickerTrait>>, table: Weak<Mutex<PickerTable>>, id: u32) -> Self {
            Self {
                picker,
                table,
                id,
            }
        }
    }
}

// 
// PickerTable
//
// A table that manages ids and tokens for picking objects;
// To use you subscribe by giving the table a function, you'll then get a PickerToken
// Once PickerToken is dropped, it (and its id) will be removed from the table
//

pub struct PickerTable {
    table: HashMap<u32, WeakPickerToken>,

    self_ref: Option<Weak<Mutex<Self>>>,
}

impl PickerTable {
    pub fn new() -> Arc<Mutex<Self>> {
        let s = Arc::new(Mutex::new(Self {
            table: HashMap::with_capacity(256),
            self_ref: None,
        }));

        s.lock().self_ref = Some(Arc::downgrade(&s));

        s
    }

    pub fn subscribe(&mut self, picker: impl Fn(u32) + Send + Sync + 'static) -> PickerToken {
        let picker: Arc<Mutex<PickerTrait>> = Arc::new(Mutex::new(picker));

        let mut num: u32 = rand::random();

        while self.table.contains_key(&num) {
            num = rand::random();
        }

        let token = PickerToken::new(
            picker.clone(),
            self.self_ref.as_ref().unwrap().clone(),
            num,
        );
        
        self.table.insert(num, WeakPickerToken::new(
            Arc::downgrade(&picker),
            self.self_ref.as_ref().unwrap().clone(),
            num,
        ));

        token
    }

    pub fn num_keys(&self) -> usize {
        self.table.keys().len()
    }
}

//
// Picker
//
// The main picker structure; it contains the texture, material, 
// and other necessary types required for rendering and picking items
// 

pub struct Picker {
    pub picking_pixel_buffer: glium::texture::pixel_buffer::PixelBuffer<u32>,
    pub picking_attachments: Option<(glium::texture::UnsignedTexture2d, glium::framebuffer::DepthRenderBuffer)>,

    table: Arc<Mutex<PickerTable>>,

    shader: ClickMaterial,
}

impl Picker {
    pub fn new(display: &dyn glium::backend::Facade) -> Self {
        let shader = crate::renderer::material::get_material_manager().get_click_material();

        Self {
            picking_pixel_buffer: glium::texture::pixel_buffer::PixelBuffer::new_empty(display, 1),
            picking_attachments: None,
            table: PickerTable::new(),
            shader,
        }
    }

    // Subscribe to an event; it will return a picker token that, once dropped, will remove it from the PickerTable
    pub fn subscribe(&mut self, picker: impl Fn(u32) + Send + Sync + 'static) -> PickerToken {
        let token = self.table.lock().subscribe(picker);

        token
    }

    //
    // Drawing Functions
    //
    // Each vertex should have an id that will be rendered to the picker's surface;
    // clicking will read the pixel that is under the cursor
    //

    pub fn clear_surface(&mut self, window: &mut impl glium::Surface, facade: &dyn glium::backend::Facade) {
        
        if self.picking_attachments.is_none() || (
            self.picking_attachments.as_ref().unwrap().0.get_width(),
            self.picking_attachments.as_ref().unwrap().0.get_height().unwrap()
        ) != window.get_dimensions() {
            let (width, height) = window.get_dimensions();

            self.picking_attachments = Some((
                glium::texture::UnsignedTexture2d::empty_with_format(
                    facade,
                    glium::texture::UncompressedUintFormat::U32,
                    glium::texture::MipmapsOption::NoMipmap,
                    width, height,
                ).unwrap(),
                glium::framebuffer::DepthRenderBuffer::new(
                    facade,
                    glium::texture::DepthFormat::F32,
                    width, height,
                ).unwrap()
            ));
        }

        //draw to textures
        if let Some((ref picking_texture, ref depth_buffer)) = &self.picking_attachments {
            //clear picking texture
            picking_texture.main_level().first_layer().into_image(None).unwrap().raw_clear_buffer([0u32, 0, 0, 0]);

            let mut picking_target = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(facade, picking_texture, depth_buffer).unwrap();
            picking_target.clear_depth(1.0);
        }
    }

    pub fn draw(&mut self, facade: &dyn glium::backend::Facade, global_transform: &WorldTransform,
        vertex_buffer: &VertexBuffer<Vertex>, index_buffer: &IndexBuffer<u32>, draw_parameters: &glium::DrawParameters<'_>) {

        //draw to textures
        if let Some((ref picking_texture, ref depth_buffer)) = &self.picking_attachments {
            //clear picking texture
            let mut picking_target = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(facade, picking_texture, depth_buffer).unwrap();

            self.shader.draw(&(&vertex_buffer, &index_buffer), &mut picking_target, global_transform, draw_parameters);
        }
    }

    pub fn click(&self, cursor: (i32, i32)) {
        if let Some(&(ref picking_texture, _)) = self.picking_attachments.as_ref() {
            let read_target = glium::Rect {
                left: (cursor.0 - 1) as u32,
                bottom: picking_texture.get_height().unwrap().saturating_sub(std::cmp::max(cursor.1 - 1, 0) as u32),
                width: 1,
                height: 1,
            };

            if read_target.left < picking_texture.get_width()
            && read_target.bottom < picking_texture.get_height().unwrap() {
                picking_texture.main_level()
                    .first_layer()
                    .into_image(None).unwrap()
                    .raw_read_to_pixel_buffer(&read_target, &self.picking_pixel_buffer);
            } else {
                self.picking_pixel_buffer.write(&[0]);
            }
        } else {
            self.picking_pixel_buffer.write(&[0]);
        }
        
        let id = self.picking_pixel_buffer.read().map(|d| d[0]).unwrap_or(0);

        if id != 0 && self.table.lock().table.contains_key(&id) {
            let picker = self.get_picker(&id);

            if let Some(callback) = &picker {
                callback.run();
            }
        }
    }

    //
    // Accessors
    //

    pub fn get_table(&self) -> Arc<Mutex<PickerTable>> {
        self.table.clone()
    }

    fn get_picker(&self, id: &u32) -> std::option::Option<PickerToken> {
        if let Some(entry) = self.table.lock().table.get(id) {

            return entry.upgrade();
        }

        None
    }
}


use crate::render::shape::Vertex;
use crate::render::material::{*};
use crate::render::matrix::{WorldTransform, Matrix};

use std::collections::HashMap;
use rand::prelude::*;
use glium::{VertexBuffer, IndexBuffer, Surface};

#[derive(Clone)]
pub struct PickerEntry {
    pub row: usize,
    pub column: usize,
    pub id: u32,
}

pub struct Picker {
    pub picking_pixel_buffer: glium::texture::pixel_buffer::PixelBuffer<u32>,
    pub picking_attachments: Option<(glium::texture::UnsignedTexture2d, glium::framebuffer::DepthRenderBuffer)>,

    table: HashMap<u32, PickerEntry>,
    random_gen: ThreadRng,
    shader: ClickMaterial,
}

impl Picker {
    pub fn new(display: &dyn glium::backend::Facade) -> Self {

        let shader = crate::render::material::material_manager::get_material_manager().get_click_material();

        Self {
            picking_pixel_buffer: glium::texture::pixel_buffer::PixelBuffer::new_empty(display, 1),
            picking_attachments: None,
            table: HashMap::with_capacity(512),
            random_gen: rand::thread_rng(),
            shader,
        }
    }

    pub fn click(&mut self, cursor: (i32, i32)) {
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
    }

    pub fn get_clicked(&mut self) -> Option<&PickerEntry> {

        let id = self.picking_pixel_buffer.read().map(|d| d[0]).unwrap_or(0);

        if id != 0 && self.table.contains_key(&id) {

            let data = self.table.get(&id).unwrap();

            return Some(data)
            
        }

        None
    }

    pub fn draw(&mut self, target: &mut impl glium::Surface, facade: &dyn glium::backend::Facade, global_transform: &WorldTransform,
        vertex_buffer: &VertexBuffer<Vertex>, index_buffer: &IndexBuffer<u32>, draw_parameters: &glium::DrawParameters<'_>) {
        // renderer.quilt.draw_click(target, global_transform, &Default::default());

        //make sure that picking attachments (picking texture, depth buffer) are valid for the current context
        if self.picking_attachments.is_none() || (
            self.picking_attachments.as_ref().unwrap().0.get_width(),
            self.picking_attachments.as_ref().unwrap().0.get_height().unwrap()
        ) != target.get_dimensions() {
            let (width, height) = target.get_dimensions();

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

            self.shader.draw(&(&vertex_buffer, &index_buffer), &mut picking_target, global_transform, &Matrix::new(), draw_parameters);
        }
    }

    pub fn get_new_id(&mut self, row: usize, column: usize) -> u32{
        let mut num: u32 = self.random_gen.gen();

        while self.table.contains_key(&num) {
            num = self.random_gen.gen()
        }
        
        self.table.insert(num, PickerEntry {row, column, id: num});

        num
    }

    pub fn remove_id(&mut self, id: u32) {
        self.table.remove(&id);
    }
}
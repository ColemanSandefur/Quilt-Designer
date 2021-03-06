pub mod block_pattern;
pub mod block_manager;

use block_pattern::BlockPattern;
use crate::program::quilt::brush::*;
use crate::renderer::shape_object::{ShapeDataStruct};
use crate::renderer::matrix::{Matrix};
use crate::renderer::vertex::Vertex;
use crate::program::update_status::{SyncUpdateStatus, WeakUpdateStatus};
use crate::renderer::picker::*;
use crate::renderer::Renderable;
use crate::parse::*;
use crate::program::quilt::protective_struct::ProtectiveStructure;

use std::sync::{Arc, Weak};
use parking_lot::Mutex;

struct ShapeProtector {
    shapes: Vec<Arc<Mutex<ShapeDataStruct>>>,
    vertex_buffer: Vec<Vertex>,
    index_buffer: Vec<u32>,
    index_count: usize,
    vertex_count: usize,
}

impl ShapeProtector {
    pub fn get_num_shapes(&self) -> usize {
        self.shapes.len()
    }

    fn new() -> Self {
        Self {
            shapes: Vec::with_capacity(10),
            vertex_buffer: Vec::new(),
            index_buffer: Vec::new(),
            vertex_count: 0,
            index_count: 0,
        }
    }

    fn update_vertex_index_counts(&mut self) {
        self.index_count = 0;
        self.vertex_count = 0;

        for shape in &mut self.shapes {
            let shape = shape.lock();
            self.index_count += shape.shape.get_num_indices();
            self.vertex_count += shape.shape.get_num_vertices();
        }
    }

    fn update_buffer(&mut self) {
        self.update_vertex_index_counts();

        let mut vert_vec = Vec::with_capacity(self.vertex_count);
        let mut index_vec = Vec::with_capacity(self.index_count);

        for shape in &mut self.shapes {
            let shape = shape.lock();
            let mut index: Vec<u32> = shape.shape.get_indices().into_iter().map(|value| value + vert_vec.len() as u32).collect();
            index_vec.append(&mut index);
            let mut vert = shape.shape.get_vertices();
            vert_vec.append(&mut vert);
        }

        self.vertex_buffer = vert_vec;
        self.index_buffer = index_vec;
    }
}

impl ProtectiveStructure<Vec<Arc<Mutex<ShapeDataStruct>>>> for ShapeProtector {
    fn modify(&mut self, modification: impl FnOnce(&mut Vec<Arc<Mutex<ShapeDataStruct>>>)) {
        modification(&mut self.shapes);
        self.update_buffer();
    }
}

impl Renderable for ShapeProtector {
    fn get_ib(&self) -> Vec<u32> {
        self.index_buffer.clone()
    }

    fn get_vb(&self) -> Vec<Vertex> {
        self.vertex_buffer.clone()
    }

    fn get_index_count(&self) -> usize {
        self.index_count
    }

    fn get_vertex_count(&self) -> usize {
        self.vertex_count
    }
}

#[derive(Clone)]
pub struct Block {
    shape_protector: Arc<Mutex<ShapeProtector>>,
    
    row: usize,
    column: usize,

    rotation: Arc<Mutex<f32>>,
    model_transform: Matrix,
    brush: Weak<Mutex<Brush>>,
}

impl Block {
    pub const BLOCK_BORDER_WIDTH: f32 = 0.05;
    pub const SHAPE_BORDER_WIDTH: f32 = 0.02;

    fn configure_click(weak_shape_protector: Weak<Mutex<ShapeProtector>>, weak_shape: Weak<Mutex<ShapeDataStruct>>, weak_brush: Weak<Mutex<Brush>>, weak_update: WeakUpdateStatus, weak_picker_table: Weak<Mutex<PickerTable>>, weak_rotation: Weak<Mutex<f32>>) -> impl Fn(u32) + Sync + Send + 'static {
        
        move |_| {

            if let Some(shape) = Weak::upgrade(&weak_shape) {
                if let Some(shape_protector) = Weak::upgrade(&weak_shape_protector) {
                    if let Some(brush) = Weak::upgrade(&weak_brush) {
                        if let Some(update) = weak_update.upgrade() {
                            if let Some(picker_table) = Weak::upgrade(&weak_picker_table) {
        
                                // ran on click

                                shape_protector.lock().modify(|vec| {
                                    let brush_lock = brush.lock();
                                    let mut shape_lock = shape.lock();
            
                                    if brush_lock.is_pattern_brush() {
                                        // change color
            
                                        brush_lock.get_pattern_brush().unwrap().apply_to_shape(&mut *shape_lock);
                                    } else {
                                        // change block pattern
            
                                        let transform = shape_lock.shape.get_model_matrix().clone();
            
                                        let new_shapes: Vec<Arc<Mutex<ShapeDataStruct>>> = brush_lock.get_block_brush().unwrap().get_pattern().get_shape_clone().into_iter().map(|shape_entry| {
                                            let shape = Arc::new(Mutex::new(*shape_entry));

                                            let token = picker_table.lock().subscribe(
                                                Self::configure_click(weak_shape_protector.clone(), Arc::downgrade(&shape), weak_brush.clone(), weak_update.clone(), weak_picker_table.clone(), weak_rotation.clone())
                                            );

                                            {
                                                let mut shape_lock = shape.lock();

                                                shape_lock.set_picker_token(Some(token));
                                                shape_lock.shape.set_model_matrix(transform.clone());
                                            }
            
                                            shape
                                        }).collect();

                                        if let Some(rotation) = Weak::upgrade(&weak_rotation) {
                                            *rotation.lock() = Brush::get_rotation();
                                        }
            
                                        vec.clear();

                                        for shape in new_shapes {
                                            vec.push(shape);
                                        }

                                        vec.last().unwrap().lock().set_picker_token(None);
                                    }
                                });

                                update.needs_updated();

                                return;
                            }
                        }
                    }
                }
                shape.lock().set_picker_token(None);
            }

        } // end of callback
        
    }

    pub fn new(row: usize, column: usize, picker: &mut Picker, brush: Arc<Mutex<Brush>>, quilt_update: SyncUpdateStatus) -> Self {
        let shape_protector = Arc::new(Mutex::new(ShapeProtector::new()));

        let shape_protector_weak = Arc::downgrade(&shape_protector.clone());
        let weak_brush = Arc::downgrade(&brush);
        let rotation = Arc::new(Mutex::new(0.0));
        shape_protector.lock().modify(|vec| {

            let shapes_vec = vec!{
                Arc::new(Mutex::new(ShapeDataStruct::new(
                    Box::new(crate::renderer::shape::PathShape::square_with_line_width(0.0, 0.0, 1.0, 1.0, 0, 0.0)),
                ))),
                Arc::new(Mutex::new(ShapeDataStruct::new(
                    Box::new(crate::renderer::shape::StrokeShape::square(0.0, 0.0, 1.0, 1.0, 0, &lyon::tessellation::StrokeOptions::default().with_line_width(Self::BLOCK_BORDER_WIDTH))),
                ))),
            };

            let picker_table = Arc::downgrade(&picker.get_table());

            for shape in shapes_vec {
                shape.lock().subscribe(picker, Self::configure_click(shape_protector_weak.clone(), Arc::downgrade(&shape), weak_brush.clone(), quilt_update.weak(), picker_table.clone(), Arc::downgrade(&rotation)));

                vec.push(shape);
            }

            vec.last().unwrap().lock().set_picker_token(None);

        });

        Self {
            shape_protector,

            row,
            column,

            rotation,
            model_transform: Matrix::new(),
            brush: Arc::downgrade(&brush),
        }
    }

    pub fn get_model_transform(&self) -> Matrix {
        self.model_transform
    }

    pub fn set_model_transform(&mut self, matrix: Matrix) {
        self.shape_protector.lock().modify(move |vec| {
            for item in vec {
                item.lock().shape.set_model_matrix(matrix);
            }
        })
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_column(&self) -> usize {
        self.column
    }

    pub fn from_save(yaml:Yaml, picker: &mut Picker, quilt_needs_updated: WeakUpdateStatus, brush: Arc<Mutex<Brush>>, save_data: &mut SaveData) -> Self {
        let map = LinkedHashMap::from(yaml);

        let row = usize::from(map.get("row"));
        let column = usize::from(map.get("column"));
        let yaml_vec: Vec<Yaml> = map.get("shapes").into();
        let rotation = Arc::new(Mutex::new(f32::from(map.get("rotation"))));

        // println!("Rotation: {}", rotation);

        let shape_protector = Arc::new(Mutex::new(ShapeProtector::new()));
        let shape_protector_weak = Arc::downgrade(&shape_protector.clone());
        let weak_brush = Arc::downgrade(&brush);
        let quilt_needs_updated = quilt_needs_updated.upgrade().unwrap();

        shape_protector.lock().modify(|vec| {
            // the save just contains the main shapes, it doesn't contain the border
            let mut new_shapes: Vec<Arc<Mutex<ShapeDataStruct>>> = yaml_vec.into_iter().map(|data| {
                let mut shape = ShapeDataStruct::from_save(data, save_data);

                shape.shape.set_rotation(*rotation.lock());

                Arc::new(Mutex::new(*shape))
            }).collect();

            // add the border to shapes
            new_shapes.push({
                let mut border = BlockPattern::get_border();

                border.shape.set_rotation(*rotation.lock());

                Arc::new(Mutex::new(*border))
            });
            
            let picker_table = Arc::downgrade(&picker.get_table());

            for shape in new_shapes {
                shape.lock().subscribe(picker, Self::configure_click(shape_protector_weak.clone(), Arc::downgrade(&shape), weak_brush.clone(), quilt_needs_updated.weak(), picker_table.clone(), Arc::downgrade(&rotation)));

                vec.push(shape);
            }

            // remove the picker token from the border
            vec.last().unwrap().lock().set_picker_token(None);
        });

        Self {
            shape_protector: shape_protector,
            row,
            column,
            rotation,
            model_transform: Matrix::new(),
            brush: Arc::downgrade(&brush)
        }
    }

    pub fn to_save(&self, save_data: &mut SaveData) -> Yaml {
        // let shapes = self.shape_protector.to_save(save_data);

        let mut vec: Vec<Yaml> = Vec::with_capacity(self.shape_protector.lock().get_num_shapes());

        self.shape_protector.lock().modify(|shapes| {
            for shape in &shapes[..shapes.len() - 1] {
                vec.push(shape.lock().to_save(save_data));
            }
        });

        LinkedHashMap::create(vec![
            ("shapes", Yaml::from(vec)),
            ("row", self.row.into()),
            ("column", self.column.into()),
            ("rotation", (*self.rotation.lock()).into()),
        ])
    }
}

impl Renderable for Block {
    fn get_ib(&self) -> Vec<u32> {
        self.shape_protector.lock().get_ib()
    }

    fn get_vb(&self) -> Vec<Vertex> {
        self.shape_protector.lock().get_vb()
    }

    fn get_index_count(&self) -> usize {
        self.shape_protector.lock().get_index_count()
    }

    fn get_vertex_count(&self) -> usize {
        self.shape_protector.lock().get_vertex_count()
    }
}
use crate::parse::{Yaml, SavableBlueprint, Savable, SaveData}; 
use crate::renderer::matrix::{Matrix};
use crate::renderer::shape::{Shape, PathShape};
use crate::renderer::picker::{Picker, PickerToken};


// Everything rendered will be a Shape Object, this will be added to the renderer's list
// the renderer will then handle the drawing of the object

pub struct ShapeDataStruct {
    pub shape: Box<dyn Shape>,
    picker: Option<PickerToken>
}

impl ShapeDataStruct {
    pub fn new(shape: Box<dyn Shape>) -> Self {
        Self {
            shape,
            picker: None,
        }
    }

    pub fn set_picker_token(&mut self, picker_token: Option<PickerToken>) {
        if picker_token.is_some() {self.shape.set_id(picker_token.as_ref().unwrap().id)} else {self.shape.set_id(0)};
        
        self.picker = picker_token;
    }

    pub fn has_picker_token(&self) -> bool {
        self.picker.is_some()
    }

    pub fn subscribe(&mut self, picker: &mut Picker, callback: impl Fn(u32) + Send + Sync + 'static) {
        self.picker = Some(picker.subscribe(callback));

        self.shape.set_id(self.picker.as_ref().unwrap().id);
    }
}

impl Clone for ShapeDataStruct {
    fn clone(&self) -> Self {
        Self {
            shape: self.shape.clone_shape(),
            picker: None,
        }
    }
}

pub trait ShapeObject {
    fn get_shapes(&self) -> &Vec<Box<ShapeDataStruct>>;
    fn get_shapes_mut(&mut self) -> &mut Vec<Box<ShapeDataStruct>>;
    fn get_model_transform(&self) -> &Matrix;
    fn get_model_transform_mut(&mut self) -> &mut Matrix;
}

impl SavableBlueprint for ShapeDataStruct {
    fn to_save_blueprint(&self) -> Yaml {
        self.shape.to_save_blueprint()
    }

    fn from_save_blueprint(yaml: Yaml) -> Box<Self> where Self: Sized {
        Box::new(Self {
            shape: PathShape::from_save_blueprint(yaml),
            picker: None,
        })
    }
}

impl Savable for ShapeDataStruct {
    fn to_save(&self, save_data: &mut SaveData) -> Yaml {
        self.shape.to_save(save_data)
    }

    fn from_save(yaml: Yaml, _save_data: &mut SaveData) -> Box<Self> where Self: Sized {
        Box::new(Self {
            shape: PathShape::from_save_blueprint(yaml),
            picker: None,
        })
    }
}
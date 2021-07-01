use crate::quilt::child_shape::ChildShape;
use crate::window::canvas::Canvas;
use crate::util::click::Click;
use crate::parser::{SavableBlueprint, Savable, Parser, Serializer, ParseData, SerializeData, SaveData};
use crate::quilt::square::Square;

use cairo::{Context};
use gdk::EventButton;
use yaml_rust::Yaml;

//
// Child shapes
//
// These will be rendered by the square, these are the different patterns that a shape might have
// They save their shape to a surface for easy rendering
//


#[derive(Clone)]
pub struct BlockPattern {
    rotation: f64,
    pattern: Vec<ChildShape>,
}

impl BlockPattern {
    pub fn new() -> Self {
        let mut pattern = Vec::new();

        pattern.push(ChildShape::new());

        Self {
            pattern,
            rotation: 0.0,
        }
    }

    pub fn new_pattern(pattern: Vec<ChildShape>) -> Self {
        Self {
            pattern,
            rotation: 0.0,
        }
    }

    pub fn apply_transformation(&self, cr: &Context) {
        cr.translate(Square::SQUARE_WIDTH / 2.0, Square::SQUARE_WIDTH / 2.0);
        cr.rotate(self.rotation);
        cr.translate(-Square::SQUARE_WIDTH / 2.0, -Square::SQUARE_WIDTH / 2.0);
    }

    pub fn draw(&self, cr: &Context) {

        self.apply_transformation(cr);

        for child in &self.pattern {
            child.draw(cr);
        }
    }

    pub fn rotate(&mut self, amount_radians: f64) {
        let rot = self.rotation;
        self.rotation = rot + amount_radians;
    }
}

impl Click for BlockPattern {
    fn click(&mut self, canvas: &Canvas, cr: &Context, event: &EventButton) -> bool {

        self.apply_transformation(cr);

        for child in &mut self.pattern {
            if child.click(canvas, cr, event) {
                return true;
            }
        }

        false
    }
}

impl SavableBlueprint for BlockPattern {
    fn from_save_blueprint(yaml_array: &yaml_rust::Yaml) -> Box<Self> {
        let yaml_array = yaml_array.as_vec().unwrap();
        let mut pattern = Vec::with_capacity(yaml_array.len());

        for yaml in yaml_array {
            pattern.push(*ChildShape::from_save_blueprint(yaml));
        }

        Box::new(Self {
            pattern,
            rotation: 0.0,
        })
    }

    fn to_save_blueprint(&self) -> yaml_rust::Yaml {
        let mut yaml = Vec::with_capacity(self.pattern.len());

        for shape in &self.pattern {
            yaml.push(shape.to_save_blueprint());
        }

        yaml_rust::Yaml::Array(yaml)
    }
}

impl Savable for BlockPattern {
    fn to_save(&self, save_path: &mut SaveData) -> Yaml {
        Serializer::create_map(vec!{
            ("rotation", Serializer::serialize(self.rotation)),
            ("pattern", Serializer::serialize(self.pattern.iter().map(|shape| shape.to_save(save_path)).collect::<Vec<Yaml>>()))
        })
    }

    fn from_save(yaml: &Yaml, save_path: &mut SaveData) -> Box<Self> {
        let map = Parser::to_map(yaml);

        let rotation = Parser::parse(map.get(&Serializer::serialize("rotation")).unwrap());
        let pattern = Parser::to_vec(map.get(&Serializer::serialize("pattern")).unwrap()).iter().map(|yaml| *ChildShape::from_save(yaml, save_path)).collect();

        Box::new(Self {
            rotation,
            pattern,
        })
    }
}
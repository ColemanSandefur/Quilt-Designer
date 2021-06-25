use cairo::{Context};
use std::sync::{Arc};
use yaml_rust::Yaml;
use crate::parser::{Parser, Serializer, Savable, SavableBlueprint};

pub trait Path: std::marker::Sync + std::marker::Send + Savable + SavableBlueprint {
    fn draw_path(&self, cr: &Context);
    fn clone_path(&self) -> Arc<dyn Path>;
}

///////////////////////////////////////////////////////////////
////      Line      ///////////////////////////////////////////
///////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Line {
    end: (f64, f64),
}

impl Line {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            end: (x,y)
        }
    }
}

impl Path for Line {
    fn draw_path(&self, cr: &Context) {
        cr.line_to(self.end.0, self.end.1);
    }

    fn clone_path(&self) -> Arc<dyn Path> {
        Arc::new(self.clone())
    }
}

impl Savable for Line {
    fn to_save(&self) -> Yaml {
        Serializer::create_map(vec![
            ("name", Serializer::from_str("line")),
            ("end", Serializer::create_map(vec![
                ("x", Serializer::from_f64(self.end.0)),
                ("y", Serializer::from_f64(self.end.1)),
            ]))
        ])
    }

    fn from_save(yaml: &Yaml) -> Box<Self> {
        let map = Parser::to_map(yaml);

        let end_map = Parser::to_map(map.get(&Yaml::from_str("end")).unwrap());
        let end = (
            Parser::to_f64(end_map.get(&Yaml::from_str("x")).unwrap()), 
            Parser::to_f64(end_map.get(&Yaml::from_str("y")).unwrap())
        );

        Box::new(Self {
            end,
        })
    }
}

impl SavableBlueprint for Line {
    fn to_save_blueprint(&self) -> Yaml {
        Serializer::create_map(vec![
            ("name", Serializer::from_str("line")),
            ("end", Serializer::create_map(vec![
                ("x", Serializer::from_f64(self.end.0)),
                ("y", Serializer::from_f64(self.end.1)),
            ]))
        ])
    }

    fn from_save_blueprint(yaml: &Yaml) -> Box<Self> {
        let map = Parser::to_map(yaml);

        let end_map = Parser::to_map(map.get(&Yaml::from_str("end")).unwrap());
        let end = (
            Parser::to_f64(end_map.get(&Yaml::from_str("x")).unwrap()), 
            Parser::to_f64(end_map.get(&Yaml::from_str("y")).unwrap())
        );

        Box::new(Self {
            end,
        })
    }
}

///////////////////////////////////////////////////////////////
////      Move      ///////////////////////////////////////////
///////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Move {
    point: (f64, f64),
}

impl Move {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            point: (x, y)
        }
    }
}

impl Path for Move {
    fn draw_path(&self, cr: &Context) {
        cr.move_to(self.point.0, self.point.1);
    }

    fn clone_path(&self) -> Arc<dyn Path> {
        Arc::new(self.clone())
    }
}

impl Savable for Move {
    fn to_save(&self) -> Yaml {
        Serializer::create_map(vec![
            ("name", Serializer::from_str("move")),
            ("point", Serializer::create_map(vec![
                ("x", Serializer::from_f64(self.point.0)),
                ("y", Serializer::from_f64(self.point.1)),
            ])),
        ])
    }

    fn from_save(yaml: &Yaml) -> Box<Self> {
        let map = Parser::to_map(yaml);

        let point_map = Parser::to_map(map.get(&Yaml::from_str("point")).unwrap());
        let point = (
            Parser::to_f64(point_map.get(&Yaml::from_str("x")).unwrap()), 
            Parser::to_f64(point_map.get(&Yaml::from_str("y")).unwrap())
        );

        Box::new(Self {
            point,
        })
    }
}

impl SavableBlueprint for Move {
    fn to_save_blueprint(&self) -> Yaml {
        Serializer::create_map(vec![
            ("name", Serializer::from_str("move")),
            ("point", Serializer::create_map(vec![
                ("x", Serializer::from_f64(self.point.0)),
                ("y", Serializer::from_f64(self.point.1)),
            ])),
        ])
    }

    fn from_save_blueprint(yaml: &Yaml) -> Box<Self> {
        let map = Parser::to_map(yaml);

        let point_map = Parser::to_map(map.get(&Yaml::from_str("point")).unwrap());
        let point = (
            Parser::to_f64(point_map.get(&Yaml::from_str("x")).unwrap()), 
            Parser::to_f64(point_map.get(&Yaml::from_str("y")).unwrap())
        );

        Box::new(Self {
            point,
        })
    }
}

///////////////////////////////////////////////////////////////
////      Arc       ///////////////////////////////////////////
///////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct ArcPath {
    center: (f64, f64),
    radius: f64,
    start_angle: f64,
    end_angle: f64,
}

impl ArcPath {
    pub fn new(xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) -> Self {
        Self {
            center: (xc, yc),
            radius,
            start_angle: angle1,
            end_angle: angle2
        }
    }
}

impl Path for ArcPath {
    fn draw_path(&self, cr: &Context) {
        cr.arc(self.center.0, self.center.1, self.radius, self.start_angle, self.end_angle);
    }

    fn clone_path(&self) -> Arc<dyn Path> {
        Arc::new(self.clone())
    }
}

impl Savable for ArcPath {
    fn to_save(&self) -> Yaml {
        Serializer::create_map(vec![
            ("name", Serializer::from_str("arc")),
            ("center", Serializer::create_map(vec![
                ("x", Serializer::from_f64(self.center.0)),
                ("y", Serializer::from_f64(self.center.1)),
            ])),
            ("radius", Serializer::from_f64(self.radius)),
            ("start_angle", Serializer::from_f64(self.start_angle)),
            ("end_angle", Serializer::from_f64(self.end_angle)),
        ])
    }

    fn from_save(yaml: &Yaml) -> Box<Self> {
        let map = Parser::to_map(yaml);

        let center = Parser::to_map(map.get(&Yaml::from_str("center")).unwrap());
        let center = (
            Parser::to_f64(center.get(&Yaml::from_str("x")).unwrap()), 
            Parser::to_f64(center.get(&Yaml::from_str("y")).unwrap())
        );
        
        let radius = Parser::to_f64(map.get(&Yaml::from_str("radius")).unwrap());
        let start_angle = Parser::to_f64(map.get(&Yaml::from_str("start_angle")).unwrap());
        let end_angle = Parser::to_f64(map.get(&Yaml::from_str("end_angle")).unwrap());

        Box::new(Self {
            center,
            radius,
            start_angle,
            end_angle
        })
    }
}

impl SavableBlueprint for ArcPath {
    fn to_save_blueprint(&self) -> Yaml {
        Serializer::create_map(vec![
            ("name", Serializer::from_str("arc")),
            ("center", Serializer::create_map(vec![
                ("x", Serializer::from_f64(self.center.0)),
                ("y", Serializer::from_f64(self.center.1)),
            ])),
            ("radius", Serializer::from_f64(self.radius)),
            ("start_angle", Serializer::from_f64(self.start_angle)),
            ("end_angle", Serializer::from_f64(self.end_angle)),
        ])
    }

    fn from_save_blueprint(yaml: &Yaml) -> Box<Self> {
        let map = Parser::to_map(yaml);

        let center = Parser::to_map(map.get(&Yaml::from_str("center")).unwrap());
        let center = (
            Parser::to_f64(center.get(&Yaml::from_str("x")).unwrap()), 
            Parser::to_f64(center.get(&Yaml::from_str("y")).unwrap())
        );
        
        let radius = Parser::to_f64(map.get(&Yaml::from_str("radius")).unwrap());
        let start_angle = Parser::to_f64(map.get(&Yaml::from_str("start_angle")).unwrap());
        let end_angle = Parser::to_f64(map.get(&Yaml::from_str("end_angle")).unwrap());

        Box::new(Self {
            center,
            radius,
            start_angle,
            end_angle
        })
    }
}

pub fn from_yaml(yaml_map: &Yaml) -> Option<Arc<dyn Path>>{
    let map = yaml_map.as_hash().unwrap();

    let name = map.get(&Yaml::from_str("name")).unwrap().as_str().unwrap();

    match name {
        "arc" => Some(Arc::new(*ArcPath::from_save_blueprint(yaml_map))),
        "move" => Some(Arc::new(*Move::from_save_blueprint(yaml_map))),
        "line" => Some(Arc::new(*Line::from_save_blueprint(yaml_map))),
        _ => None
    }
}
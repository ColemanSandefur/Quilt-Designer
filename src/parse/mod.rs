use yaml_rust::Yaml as YamlRust;
use linked_hash_map::LinkedHashMap as LinkedHashMapRust;
use std::ops::{Deref, DerefMut};
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Write;
use lyon::math::{point, Point};

// yaml wrapper
#[derive(Clone, Hash, std::cmp::PartialEq, std::cmp::Eq)]
pub struct Yaml(YamlRust);

impl Yaml {
    pub fn print(&self) {
        println!("=== dump: ===\n{}\n=============\n", self.dump_to_string());
    }

    pub fn load_from_file(path: &Path) -> Self {
        let mut file = File::open(path).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        yaml_rust::YamlLoader::load_from_str(&contents).unwrap().remove(0).into()
    }

    pub fn save_to_file(&self, path: &Path) {
        let mut file = File::create(path).expect("Error creating file to save");

        file.write(self.dump_to_string().as_bytes()).expect("Error writing file");
    }

    pub fn dump_to_string(&self) -> String {
        let mut output = String::new();
        let mut emitter = yaml_rust::YamlEmitter::new(&mut output);
        emitter.dump(&self).unwrap();

        output
    }
}

// conversions between wrapper and yaml crate

impl From<YamlRust> for Yaml {
    fn from(yaml: YamlRust) -> Self {
        Yaml(yaml)
    }
}

impl From<Yaml> for YamlRust {
    fn from(yaml: Yaml) -> Self {
        yaml.0
    }
}

impl AsRef<YamlRust> for Yaml {
    fn as_ref(&self) -> &YamlRust {
        &self.0
    }
}

impl Deref for Yaml {
    type Target = YamlRust;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Yaml {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// hash map wrapper

pub struct LinkedHashMap {
    pub linked_hash_map: LinkedHashMapRust<Yaml, Yaml>
}

impl LinkedHashMap {
    pub fn get(&self, key: &str) -> &Yaml {
        self.linked_hash_map.get(&YamlRust::from_str(key).into()).expect(&format!("could not find key: {}", key))
    }

    pub fn create<T: Into<Yaml>>(data: Vec<(&str, T)>) -> Yaml {
        let mut map: LinkedHashMapRust<YamlRust, YamlRust> = LinkedHashMapRust::with_capacity(data.len());

        for (key, value) in data {
            map.insert(YamlRust::String(String::from(key)), YamlRust::from(value.into()));
        }

        YamlRust::Hash(map).into()
    }
}

// hash map conversions

impl From<LinkedHashMapRust<Yaml, Yaml>> for LinkedHashMap {
    fn from(map: LinkedHashMapRust<Yaml, Yaml>) -> Self {
        Self {
            linked_hash_map: map
        }
    }
}

impl From<LinkedHashMapRust<YamlRust, YamlRust>> for LinkedHashMap {
    fn from(map: LinkedHashMapRust<YamlRust, YamlRust>) -> Self {
        let map = map.into_iter().map(|y| (y.0.into(), y.1.into())).collect();

        Self {
            linked_hash_map: map
        }
    }
}

impl From<LinkedHashMap> for LinkedHashMapRust<Yaml, Yaml> {
    fn from(map: LinkedHashMap) -> Self {
        map.linked_hash_map
    }
}

//
// converting from yaml
//

impl From<&Yaml> for bool {
    fn from(yaml: &Yaml) -> Self {
        yaml.as_bool().unwrap()
    }
}

impl From<Yaml> for bool {
    fn from(yaml: Yaml) -> Self {
        yaml.into()
    }
}

impl From<&Yaml> for i32 {
    fn from(yaml: &Yaml) -> Self {
        i64::from(yaml) as i32
    }
}

impl From<Yaml> for i32 {
    fn from(yaml: Yaml) -> Self {
        yaml.into()
    }
}

impl From<&Yaml> for i64 {
    fn from(yaml: &Yaml) -> Self {
        yaml.as_i64().unwrap()
    }
}

impl From<Yaml> for i64 {
    fn from(yaml: Yaml) -> Self {
        yaml.into()
    }
}

impl From<&Yaml> for usize {
    fn from(yaml: &Yaml) -> Self {
        i64::from(yaml) as usize
    }
}

impl From<Yaml> for usize {
    fn from(yaml: Yaml) -> Self {
        yaml.into()
    }
}

impl From<&Yaml> for f32 {
    fn from(yaml: &Yaml) -> Self {
        f64::from(yaml) as f32
    }
}

impl From<Yaml> for f32 {
    fn from(yaml: Yaml) -> Self {
        yaml.into()
    }
}

impl From<&Yaml> for f64 {
    fn from(yaml: &Yaml) -> Self {
        if let Some(number) = yaml.as_i64() {
            return number as f64;
        }
        
        if let Some(number) = yaml.as_f64() {
            return number;
        }
        
        yaml.as_str().unwrap().parse().unwrap()
    }
}

impl From<Yaml> for f64 {
    fn from(yaml: Yaml) -> Self {
        yaml.into()
    }
}

impl From<&Yaml> for String {
    fn from(yaml: &Yaml) -> Self {
        yaml.as_str().unwrap().into()
    }
}

impl<T: From<Yaml>> From<Yaml> for Vec<T> {
    fn from(yaml: Yaml) -> Self {
        yaml.0.into_vec().unwrap().into_iter().map(|element| T::from(element.into())).collect()
    }
}

impl<T: From<Yaml>> From<&Yaml> for Vec<T> {
    fn from(yaml: &Yaml) -> Self {
        yaml.as_vec().unwrap().into_iter().map(|element| T::from(element.clone().into())).collect()
    }
}

impl From<Yaml> for LinkedHashMap {
    fn from(yaml: Yaml) -> Self {
        yaml.0.into_hash().unwrap().into()
    }
}

impl From<&Yaml> for LinkedHashMap {
    fn from(yaml: &Yaml) -> Self {
        yaml.as_hash().unwrap().clone().into()
    }
}

impl From<Yaml> for Point {
    fn from(yaml: Yaml) -> Self {
        let map = LinkedHashMap::from(yaml);

        point(map.get("x").into(), map.get("y").into())
    }
}

impl From<&Yaml> for Point {
    fn from(yaml: &Yaml) -> Self {
        let map = LinkedHashMap::from(yaml);

        point(map.get("x").into(), map.get("y").into())
    }
}

//
// converting to yaml
//

impl From<bool> for Yaml {
    fn from(data: bool) -> Self {
        Yaml(YamlRust::Boolean(data))
    }
}

impl From<i32> for Yaml {
    fn from(data: i32) -> Self {
        Yaml::from(data as i64)
    }
}

impl From<i64> for Yaml {
    fn from(data: i64) -> Self {
        Yaml(YamlRust::Integer(data))
    }
}

impl From<usize> for Yaml {
    fn from(data: usize) -> Self {
        (data as i64).into()
    }
}

impl From<f32> for Yaml {
    fn from(data: f32) -> Self {
        Yaml::from(data as f64)
    }
}

impl From<f64> for Yaml {
    fn from(data: f64) -> Self {
        Yaml(YamlRust::Real(data.to_string()))
    }
}

impl From<String> for Yaml {
    fn from(data: String) -> Self {
        YamlRust::String(data).into()
    }
}

impl From<&str> for Yaml {
    fn from(data: &str) -> Self {
        String::from(data).into()
    }
}

impl From<Point> for Yaml {
    fn from(data: Point) -> Self {
        LinkedHashMap::create(vec![
            ("x", data.x),
            ("y", data.y)
        ])
    }
}

impl<T: Into<Yaml>> From<Vec<T>> for Yaml {
    fn from(data: Vec<T>) -> Self {
        YamlRust::Array(data.into_iter().map(|data| {Into::<Yaml>::into(data).0}).collect()).into()
    }
}

pub trait SavableBlueprint {
    fn to_save_blueprint(&self) -> Yaml;
    fn from_save_blueprint(yaml: Yaml) -> Box<Self> where Self: Sized;
}

pub struct SaveData {
    pub writer: Option<zip::ZipWriter<std::fs::File>>,
    pub reader: Option<zip::ZipArchive<std::fs::File>>,
    pub files_written: Vec<String>,
}

pub trait Savable {
    fn to_save(&self, save_data: &mut SaveData) -> Yaml;
    fn from_save(yaml: Yaml, save_data: &mut SaveData) -> Box<Self> where Self: Sized;
}
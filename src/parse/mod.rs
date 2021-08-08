use yaml_rust::Yaml as YamlRust;
use linked_hash_map::LinkedHashMap as LinkedHashMapRust;
use std::ops::Deref;
use std::fs::File;
use std::path::Path;
use std::io::Read;

// yaml wrapper
#[derive(Clone, Hash, std::cmp::PartialEq, std::cmp::Eq)]
pub struct Yaml {
    pub yaml: YamlRust
}

impl Yaml {
    pub fn print(&self) {
        let mut output = String::new();
        let mut emitter = yaml_rust::YamlEmitter::new(&mut output);
        emitter.dump(&self).unwrap();
        println!("=== dump: ===\n {}\n=============\n",output);
    }

    pub fn load_from_file(path: &Path) -> Self {
        let mut file = File::open(path).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        yaml_rust::YamlLoader::load_from_str(&contents).unwrap().remove(0).into()
    }
}

// conversions between wrapper and yaml crate

impl From<YamlRust> for Yaml {
    fn from(yaml: YamlRust) -> Self {
        Yaml {
            yaml
        }
    }
}

impl From<Yaml> for YamlRust {
    fn from(yaml: Yaml) -> Self {
        yaml.yaml
    }
}

impl AsRef<YamlRust> for Yaml {
    fn as_ref(&self) -> &YamlRust {
        &self.yaml
    }
}


// hash map wrapper

pub struct LinkedHashMap {
    pub linked_hash_map: LinkedHashMapRust<Yaml, Yaml>
}

impl LinkedHashMap {
    pub fn get(&self, key: &str) -> &Yaml {
        self.linked_hash_map.get(&YamlRust::from_str(key).into()).unwrap()
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

pub trait Parse<T> {
    fn parse(&self) -> T;
}

impl Parse<f64> for Yaml {
    fn parse(&self) -> f64 {
        if let Some(number) = self.as_i64() {
            return number as f64;
        }
        
        if let Some(number) = self.as_f64() {
            return number;
        }
        
        self.as_str().unwrap().parse().unwrap()
    }
}

impl Parse<i64> for Yaml {
    fn parse(&self) -> i64 {
        self.yaml.as_i64().unwrap()
    }
}

impl Parse<bool> for Yaml {
    fn parse(&self) -> bool {
        self.yaml.as_bool().unwrap()
    }
}

impl Parse<String> for Yaml {
    fn parse(&self) -> String {
        self.yaml.as_str().unwrap().into()
    }
}

impl Deref for Yaml {
    type Target = YamlRust;

    fn deref(&self) -> &Self::Target {
        &self.yaml
    }
}

impl From<Yaml> for LinkedHashMap {
    fn from(yaml: Yaml) -> Self {
        yaml.yaml.into_hash().unwrap().into()
    }
}

impl From<Yaml> for Vec<Yaml> {
    fn from(yaml: Yaml) -> Self {
        yaml.yaml.into_vec().unwrap().into_iter().map(|element| element.into()).collect()
    }
}

pub trait SavableBlueprint {
    // fn to_save_blueprint(&self) -> Yaml;
    fn from_save_blueprint(yaml: Yaml) -> Box<Self> where Self: Sized;
}
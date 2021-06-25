use yaml_rust::Yaml;
use linked_hash_map::LinkedHashMap;

pub struct Parser {}

impl Parser {

    pub fn to_f64(yaml: &Yaml) -> f64 {
        if let Some(number) = yaml.as_i64() {
            return number as f64;
        }
    
        if let Some(number) = yaml.as_f64() {
            return number;
        }
    
        yaml.as_str().unwrap().parse().unwrap()
    }

    pub fn to_str(yaml: &Yaml) -> &str {
        yaml.as_str().unwrap()
    }

    pub fn to_map(yaml: &Yaml) -> &LinkedHashMap<Yaml, Yaml> {
        yaml.as_hash().unwrap()
    }

    pub fn print(yaml: &Yaml) {
        let mut output = String::new();
        let mut emitter = yaml_rust::YamlEmitter::new(&mut output);
        emitter.dump(&yaml).unwrap();
        println!("dump: {}",output);
    }
}

pub struct Serializer {}

impl Serializer {
    pub fn create_map(data: Vec<(&str, Yaml)>) -> Yaml {
        let mut map = LinkedHashMap::with_capacity(data.len());

        for (key, value) in data {
            map.insert(Yaml::from_str(key), value);
        }

        Yaml::Hash(map)
    }

    pub fn from_str(string: &str) -> Yaml {
        Yaml::from_str(string)
    }

    pub fn from_f64(value: f64) -> Yaml {
        Yaml::Real(value.to_string())
    }
}
use yaml_rust::Yaml;
use linked_hash_map::LinkedHashMap;

pub struct Parser {}

pub trait ParseData<T> {
    fn parse(yaml: &Yaml) -> T;
}

impl ParseData<f64> for Parser {
    fn parse(yaml: &Yaml) -> f64 {
        if let Some(number) = yaml.as_i64() {
            return number as f64;
        }
    
        if let Some(number) = yaml.as_f64() {
            return number;
        }
    
        yaml.as_str().unwrap().parse().unwrap()
    }
}

impl ParseData<i64> for Parser {
    fn parse(yaml: &Yaml) -> i64 {
        yaml.as_i64().unwrap()
    }
}

impl ParseData<usize> for Parser {
    fn parse(yaml: &Yaml) -> usize {
        yaml.as_i64().unwrap() as usize
    }
}

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

    pub fn to_i64(yaml: &Yaml) -> i64  {
        yaml.as_i64().unwrap()
    }

    pub fn to_str(yaml: &Yaml) -> &str {
        yaml.as_str().unwrap()
    }

    pub fn to_map(yaml: &Yaml) -> &LinkedHashMap<Yaml, Yaml> {
        yaml.as_hash().unwrap()
    }

    pub fn to_vec(yaml: &Yaml) -> &Vec<Yaml> {
        yaml.as_vec().unwrap()
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

    pub fn from_i64(value: i64) -> Yaml {
        Yaml::Integer(value)
    }
}

pub trait SerializeData<T> {
    fn serialize(value: T) -> Yaml;
}

impl SerializeData<f64> for Serializer {
    fn serialize(value: f64) -> Yaml {
        Yaml::Real(value.to_string())
    }
}

impl SerializeData<i64> for Serializer {
    fn serialize(value: i64) -> Yaml {
        Yaml::Integer(value)
    }
}

impl SerializeData<&str> for Serializer {
    fn serialize(value: &str) -> Yaml {
        Yaml::from_str(value)
    }
}

impl SerializeData<Vec<Yaml>> for Serializer {
    fn serialize(value: Vec<Yaml>) -> Yaml {
        Yaml::Array(value)
    }
}

impl SerializeData<usize> for Serializer {
    fn serialize(value: usize) -> Yaml {
        Serializer::serialize(value as i64)
    }
}

// save all the parts to completely re-create the file
pub trait Savable {
    fn to_save(&self, save_path: &str) -> Yaml;
    fn from_save(yaml: &Yaml, save_path: &str) -> Box<Self> where Self: Sized;
}

// save what is needed to keep the general shape, used for saving BlockPatterns
pub trait SavableBlueprint {
    fn to_save_blueprint(&self) -> Yaml;
    fn from_save_blueprint(yaml: &Yaml) -> Box<Self> where Self: Sized;
}
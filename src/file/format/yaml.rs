use yaml_rust as yaml;
use source::Source;
use std::error::Error;
use std::fmt;
use std::collections::{BTreeMap, HashMap};
use std::mem;
use value::{Value, ValueKind};

pub fn parse(uri: Option<&String>, text: &str) -> Result<HashMap<String, Value>, Box<Error>> {
    // Parse a YAML object from file
    let mut docs = yaml::YamlLoader::load_from_str(text)?;
    let root = match docs.len() {
        0 => yaml::Yaml::Hash(BTreeMap::new()),
        1 => mem::replace(&mut docs[0], yaml::Yaml::Null),
        n => {
            return Err(Box::new(MultipleDocumentsError(n)));
        }
    };

    // TODO: Have a proper error fire if the root of a file is ever not a Table
    let value = from_yaml_value(uri, &root);
    match value.kind {
        ValueKind::Table(map) => Ok(map),

        _ => Ok(HashMap::new()),
    }
}

fn from_yaml_value(uri: Option<&String>, value: &yaml::Yaml) -> Value {
    match *value {
        yaml::Yaml::String(ref value) => Value::new(uri, ValueKind::String(value.clone())),
        yaml::Yaml::Real(ref value) => {
            Value::new(uri, ValueKind::Float(value.parse::<f64>().unwrap()))
        }
        yaml::Yaml::Integer(value) => Value::new(uri, ValueKind::Integer(value)),
        yaml::Yaml::Boolean(value) => Value::new(uri, ValueKind::Boolean(value)),
        yaml::Yaml::Hash(ref table) => {
            let mut m = HashMap::new();
            for (key, value) in table {
                if let Some(k) = key.as_str() {
                    m.insert(k.to_lowercase().to_owned(), from_yaml_value(uri, value));
                }
                // TODO: should we do anything for non-string keys?
            }
            Value::new(uri, ValueKind::Table(m))
        }
        yaml::Yaml::Array(ref array) => {
            let mut l = Vec::new();

            for value in array {
                l.push(from_yaml_value(uri, value));
            }

            Value::new(uri, ValueKind::Array(l))
        }

        yaml::Yaml::Null => Value::new(uri, ValueKind::Nil),

        // TODO: how should we BadValue?
        _ => {
            unimplemented!();
        }

    }
}

#[derive(Debug, Copy, Clone)]
struct MultipleDocumentsError(usize);

impl fmt::Display for MultipleDocumentsError {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(format, "Got {} YAML documents, expected 1", self.0)
    }
}

impl Error for MultipleDocumentsError {
    fn description(&self) -> &str {
        "More than one YAML document provided"
    }
}

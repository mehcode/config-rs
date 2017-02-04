use yaml_rust as yaml;

use source::Source;
use std::error::Error;
use std::fmt;
use std::collections::{BTreeMap, HashMap};
use std::mem;
use value::Value;


pub struct Content {
    // Root table of the YAML document
    root: yaml::Yaml
}

impl Content {
    pub fn parse(text: &str) -> Result<Box<Source>, Box<Error>> {
        let mut docs = yaml::YamlLoader::load_from_str(text)?;

        match docs.len() {
            0 => Ok(Box::new(Content { root: yaml::Yaml::Hash(BTreeMap::new()) })),
            1 => Ok(Box::new(Content {
                root: mem::replace(&mut docs[0], yaml::Yaml::Null)
            })),
            n => Err(Box::new(MultipleDocumentsError(n)))
        }
    }

    pub fn from_yaml(doc: yaml::Yaml) -> Content {
        Content { root: doc }
    }
}

fn from_yaml_value<'a>(value: &yaml::Yaml) -> Value {
    match *value {
        yaml::Yaml::String(ref value) => Value::String(value.clone()),
        yaml::Yaml::Real(ref value) => Value::Float(value.parse::<f64>().unwrap()),
        yaml::Yaml::Integer(value) => Value::Integer(value),
        yaml::Yaml::Boolean(value) => Value::Boolean(value),
        yaml::Yaml::Hash(ref table) => {
            let mut m = HashMap::new();
            for (key, value) in table {
                if let Some(k) = key.as_str() {
                    m.insert(k.to_owned(), from_yaml_value(value));
                }
                // TODO: should we do anything for non-string keys?
            }
            Value::Table(m)
        }
        yaml::Yaml::Array(ref array) => {
            let l: Vec<Value> = array.iter().map(from_yaml_value).collect();
            Value::Array(l)
        }
        // TODO: how should we handle Null and BadValue?
        _ => { unimplemented!(); }

    }
}

impl Source for Content {
    fn collect(&self) -> HashMap<String, Value> {
        if let Value::Table(table) = from_yaml_value(&self.root) {
            table
        } else {
            // TODO: Better handle a non-object at root
            // NOTE: I never want to support that but a panic is bad
            panic!("expected object at YAML root");
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

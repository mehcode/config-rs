use toml;
use source::Source;
use std::collections::{HashMap, BTreeMap};
use std::error::Error;
use value::Value;

pub struct Content {
    // Root table of the TOML document
    root: toml::Value,
}

impl Content {
    pub fn parse(text: &str, namespace: Option<&String>) -> Result<Box<Source + Send + Sync>, Box<Error>> {
        // Parse
        let mut parser = toml::Parser::new(text);
        // TODO: Get a solution to make this return an Error-able
        let mut root = parser.parse().unwrap();

        // Limit to namespace
        if let Some(namespace) = namespace {
            if let Some(toml::Value::Table(table)) = root.remove(namespace) {
                root = table;
            } else {
                // TODO: Warn?
                root = BTreeMap::new();
            }
        }

        Ok(Box::new(Content { root: toml::Value::Table(root) }))
    }
}

fn from_toml_value(value: &toml::Value) -> Value {
    match *value {
        toml::Value::String(ref value) => Value::String(value.clone()),
        toml::Value::Float(value) => Value::Float(value),
        toml::Value::Integer(value) => Value::Integer(value),
        toml::Value::Boolean(value) => Value::Boolean(value),

        toml::Value::Table(ref table) => {
            let mut m = HashMap::new();

            for (key, value) in table {
                m.insert(key.clone(), from_toml_value(value));
            }

            Value::Table(m)
        }

        toml::Value::Array(ref array) => {
            let mut l = Vec::new();

            for value in array {
                l.push(from_toml_value(value));
            }

            Value::Array(l)
        }

        _ => {
            unimplemented!();
        }
    }
}

impl Source for Content {
    fn collect(&self) -> HashMap<String, Value> {
        if let Value::Table(table) = from_toml_value(&self.root) {
            table
        } else {
            unreachable!();
        }
    }
}

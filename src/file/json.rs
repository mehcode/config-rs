use serde_json;

use source::Source;
use std::error::Error;
use std::collections::HashMap;
use value::Value;

pub struct Content {
    // Root table of the TOML document
    root: serde_json::Value,
}

impl Content {
    pub fn parse(text: &str, namespace: Option<&String>) -> Result<Box<Source + Send + Sync>, Box<Error>> {
        // Parse
        let mut root: serde_json::Value = serde_json::from_str(text)?;

        // Limit to namespace
        if let Some(namespace) = namespace {
            if let serde_json::Value::Object(mut root_map) = root {
                if let Some(value) = root_map.remove(namespace) {
                    root = value;
                } else {
                    // TODO: Warn?
                    root = serde_json::Value::Object(serde_json::Map::new());
                }
            }
        }

        Ok(Box::new(Content { root: root }))
    }
}

fn from_json_value(value: &serde_json::Value) -> Value {
    match *value {
        serde_json::Value::String(ref value) => Value::String(value.clone()),

        serde_json::Value::Number(ref value) => {
            if let Some(value) = value.as_i64() {
                Value::Integer(value)
            } else if let Some(value) = value.as_f64() {
                Value::Float(value)
            } else {
                unreachable!();
            }
        }

        serde_json::Value::Bool(value) => Value::Boolean(value),

        serde_json::Value::Object(ref table) => {
            let mut m = HashMap::new();

            for (key, value) in table {
                m.insert(key.clone(), from_json_value(value));
            }

            Value::Table(m)
        }

        serde_json::Value::Array(ref array) => {
            let mut l = Vec::new();

            for value in array {
                l.push(from_json_value(value));
            }

            Value::Array(l)
        }

        // TODO: What's left is JSON Null; how should we handle that?
        _ => {
            unimplemented!();
        }
    }
}

impl Source for Content {
    fn collect(&self) -> HashMap<String, Value> {
        if let Value::Table(table) = from_json_value(&self.root) {
            table
        } else {
            // TODO: Better handle a non-object at root
            // NOTE: I never want to support that but a panic is bad
            panic!("expected object at JSON root");
        }
    }
}

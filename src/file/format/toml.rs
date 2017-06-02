use toml;
use source::Source;
use std::collections::{HashMap, BTreeMap};
use std::error::Error;
use value::Value;

pub fn parse(uri: Option<&String>, text: &str, namespace: Option<&String>) -> Result<Value, Box<Error>> {
    // Parse a TOML value from the provided text
    let mut root: toml::Value = toml::from_str(text)?;

    // Limit to namespace
    if let Some(namespace) = namespace {
        root = toml::Value::Table(match root {
            toml::Value::Table(ref mut table) => {
                if let Some(toml::Value::Table(table)) = table.remove(namespace) {
                    table
                } else {
                    BTreeMap::new()
                }
            }

            _ => {
                BTreeMap::new()
            }
        });
    }

    Ok(from_toml_value(uri, &root))
}

// TODO: Extend value origin with line/column numbers when able
fn from_toml_value(uri: Option<&String>, value: &toml::Value) -> Value {
    match *value {
        toml::Value::String(ref value) => Value::new(uri, value.to_string()),
        toml::Value::Float(value) => Value::new(uri, value),
        toml::Value::Integer(value) => Value::new(uri, value),
        toml::Value::Boolean(value) => Value::new(uri, value),

        toml::Value::Table(ref table) => {
            let mut m = HashMap::new();

            for (key, value) in table {
                m.insert(key.clone(), from_toml_value(uri, value));
            }

            Value::new(uri, m)
        }

        toml::Value::Array(ref array) => {
            let mut l = Vec::new();

            for value in array {
                l.push(from_toml_value(uri, value));
            }

            Value::new(uri, l)
        }

        _ => {
            // TODO: DateTime
            unimplemented!();
        }
    }
}

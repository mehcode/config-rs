use serde_derive::Deserialize;

use std::collections::HashMap;
use std::error::Error;

use crate::error::{ConfigError, Unexpected};
use crate::value::{Value, ValueKind};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Val {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Val>),
    Object(HashMap<String, Val>),
}

pub fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<HashMap<String, Value>, Box<dyn Error + Send + Sync>> {
    let root = json5_rs::from_str::<Val>(&text)?;
    if let Some(err) = match root {
        Val::String(ref value) => Some(Unexpected::Str(value.clone())),
        Val::Integer(value) => Some(Unexpected::Integer(value)),
        Val::Float(value) => Some(Unexpected::Float(value)),
        Val::Boolean(value) => Some(Unexpected::Bool(value)),
        Val::Object(_) => None,
        Val::Array(_) => Some(Unexpected::Seq),
        Val::Null => Some(Unexpected::Unit),
    } {
        return Err(ConfigError::invalid_root(uri, err));
    }

    let value = from_json5_value(uri, root);
    match value.kind {
        ValueKind::Table(map) => Ok(map),

        _ => Ok(HashMap::new()),
    }
}

fn from_json5_value(uri: Option<&String>, value: Val) -> Value {
    match value {
        Val::String(v) => Value::new(uri, ValueKind::String(v)),

        Val::Integer(v) => Value::new(uri, ValueKind::Integer(v)),

        Val::Float(v) => Value::new(uri, ValueKind::Float(v)),

        Val::Boolean(v) => Value::new(uri, ValueKind::Boolean(v)),

        Val::Object(table) => {
            let mut m = HashMap::new();

            for (key, value) in table {
                m.insert(key, from_json5_value(uri, value));
            }

            Value::new(uri, ValueKind::Table(m))
        }

        Val::Array(array) => {
            let mut l = Vec::new();

            for value in array {
                l.push(from_json5_value(uri, value));
            }

            Value::new(uri, ValueKind::Array(l))
        }

        Val::Null => Value::new(uri, ValueKind::Nil),
    }
}

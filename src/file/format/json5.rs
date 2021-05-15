use std::collections::HashMap;
use std::error::Error;

use crate::error::{ConfigError, Unexpected};
use crate::value::{Value, ValueKind};

#[derive(serde::Deserialize, Debug)]
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
    match json5_rs::from_str::<Val>(&text)? {
        Val::String(ref value) => Err(Unexpected::Str(value.clone())),
        Val::Integer(value) => Err(Unexpected::Integer(value)),
        Val::Float(value) => Err(Unexpected::Float(value)),
        Val::Boolean(value) => Err(Unexpected::Bool(value)),
        Val::Array(_) => Err(Unexpected::Seq),
        Val::Null => Err(Unexpected::Unit),
        Val::Object(o) => match from_json5_value(uri, Val::Object(o)).kind {
            ValueKind::Table(map) => Ok(map),
            _ => Ok(HashMap::new()),
        },
    }
    .map_err(|err| ConfigError::invalid_root(uri, err))
    .map_err(|err| Box::new(err) as Box<dyn Error + Send + Sync>)
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

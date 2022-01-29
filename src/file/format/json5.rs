use std::error::Error;

use crate::error::{ConfigError, Unexpected};
use crate::map::Map;
use crate::value::{Value, ValueKind};

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum Val {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Self>),
    Object(Map<String, Self>),
}

pub fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    match json5_rs::from_str::<Val>(text)? {
        Val::String(ref value) => Err(Unexpected::Str(value.clone())),
        Val::Integer(value) => Err(Unexpected::I64(value)),
        Val::Float(value) => Err(Unexpected::Float(value)),
        Val::Boolean(value) => Err(Unexpected::Bool(value)),
        Val::Array(_) => Err(Unexpected::Seq),
        Val::Null => Err(Unexpected::Unit),
        Val::Object(o) => match from_json5_value(uri, Val::Object(o)).kind {
            ValueKind::Table(map) => Ok(map),
            _ => Ok(Map::new()),
        },
    }
    .map_err(|err| ConfigError::invalid_root(uri, err))
    .map_err(|err| Box::new(err) as Box<dyn Error + Send + Sync>)
}

fn from_json5_value(uri: Option<&String>, value: Val) -> Value {
    let vk = match value {
        Val::Null => ValueKind::Nil,
        Val::String(v) => ValueKind::String(v),
        Val::Integer(v) => ValueKind::I64(v),
        Val::Float(v) => ValueKind::Float(v),
        Val::Boolean(v) => ValueKind::Boolean(v),
        Val::Object(table) => {
            let m = table
                .into_iter()
                .map(|(k, v)| (k, from_json5_value(uri, v)))
                .collect();

            ValueKind::Table(m)
        }

        Val::Array(array) => {
            let l = array
                .into_iter()
                .map(|v| from_json5_value(uri, v))
                .collect();

            ValueKind::Array(l)
        }
    };

    Value::new(uri, vk)
}

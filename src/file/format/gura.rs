use std::error::Error;

use gura::GuraType;

use crate::map::Map;
use crate::value::{Value, ValueKind};

pub fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    let value = from_gura_value(uri, gura::parse(text).unwrap());
    match value.kind {
        ValueKind::Table(map) => Ok(map),

        _ => Ok(Map::new()),
    }
}

fn from_gura_value(uri: Option<&String>, value: GuraType) -> Value {
    match value {
        GuraType::String(value) => Value::new(uri, ValueKind::String(value)),

        GuraType::Integer(value) => Value::new(uri, ValueKind::I64(value as i64)),
        GuraType::BigInteger(value) => Value::new(uri, ValueKind::I128(value)),
        GuraType::Float(value) => Value::new(uri, ValueKind::Float(value)),

        GuraType::Bool(value) => Value::new(uri, ValueKind::Boolean(value)),

        GuraType::Object(table) => {
            let mut m = Map::new();

            for (key, value) in table {
                m.insert(key, from_gura_value(uri, value));
            }

            Value::new(uri, ValueKind::Table(m))
        }

        GuraType::Array(array) => {
            let mut l = Vec::new();

            for value in array {
                l.push(from_gura_value(uri, value));
            }

            Value::new(uri, ValueKind::Array(l))
        }

        // Null or remaining types (only intended for internal use):
        GuraType::Null => Value::new(uri, ValueKind::Nil),
        _ => Value::new(uri, ValueKind::Nil),
    }
}

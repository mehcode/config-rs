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
    let vk = match value {
        GuraType::String(value) => ValueKind::String(value),

        GuraType::Integer(value) => ValueKind::I64(value as i64),
        GuraType::BigInteger(value) => ValueKind::I128(value),
        GuraType::Float(value) => ValueKind::Float(value),

        GuraType::Bool(value) => ValueKind::Boolean(value),

        GuraType::Object(table) => {
            let m = table
                .into_iter()
                .map(|(k, v)| (k, from_gura_value(uri, v)))
                .collect();

            ValueKind::Table(m)
        }

        GuraType::Array(array) => {
            let l = array
                .into_iter()
                .map(|v| from_gura_value(uri, v))
                .collect();

            ValueKind::Array(l)
        }

        GuraType::Null => ValueKind::Nil,

        // Remaining types (only intended for internal use):
        _ => ValueKind::Nil,
    };

    Value::new(uri, vk)
}

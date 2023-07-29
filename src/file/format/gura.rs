use std::error::Error;

use gura::GuraType;

use crate::map::Map;
use crate::value::{Value, ValueKind};

pub fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    let val = from_gura_value(uri, &gura::parse(text).unwrap());

    match val.kind {
        ValueKind::Table(map) => Ok(map),

        _ => Ok(Map::new())
    }
}

fn from_gura_value(uri: Option<&String>, val: &GuraType) -> Value {
    match val {
        GuraType::Null => Value::new(uri, ValueKind::Nil),

        GuraType::Object(ref table) => {
            let mut m = Map::new();

            for (key, val) in table {
                m.insert(key.clone(), from_gura_value(uri, val));
            }

            Value::new(uri, ValueKind::Table(m))
        }

        GuraType::Bool(val) => Value::new(uri, ValueKind::Boolean(*val)),

        GuraType::String(ref val) => Value::new(uri, ValueKind::String(val.clone())),

        GuraType::Integer(val) => Value::new(uri, ValueKind::I64(*val as i64)),

        GuraType::BigInteger(val) => Value::new(uri, ValueKind::I128(*val)),

        GuraType::Float(val) => Value::new(uri, ValueKind::Float(*val)),

        GuraType::Array(ref arr) => {
            let mut l = Vec::new();

            for val in arr {
                l.push(from_gura_value(uri, val));
            }

            Value::new(uri, ValueKind::Array(l))
        } 

        _ => Value::new(uri, ValueKind::Nil),
    }
}
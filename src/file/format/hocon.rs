use hocon;
use source::Source;
use std::collections::HashMap;
use std::error::Error;
use value::{Value, ValueKind};

pub fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<HashMap<String, Value>, Box<Error + Send + Sync>> {
    let loaded = if let Some(uri) = uri {
        hocon::HoconLoader::new().load_file(uri)
    } else {
        hocon::HoconLoader::new().load_str(text)
    };
    let value = from_hocon_value(uri, &loaded?.hocon()?);

    match value.kind {
        ValueKind::Table(map) => Ok(map),
        _ => Ok(HashMap::new()),
    }
}

fn from_hocon_value(uri: Option<&String>, value: &hocon::Hocon) -> Value {
    match *value {
        hocon::Hocon::Integer(ref value) => Value::new(uri, ValueKind::Integer(*value)),

        hocon::Hocon::Real(ref value) => Value::new(uri, ValueKind::Float(*value)),

        hocon::Hocon::Boolean(ref value) => Value::new(uri, ValueKind::Boolean(*value)),

        hocon::Hocon::String(ref value) => Value::new(uri, ValueKind::String(value.clone())),

        hocon::Hocon::Hash(ref table) => {
            let mut m = HashMap::new();

            for (key, value) in table {
                m.insert(key.to_lowercase().clone(), from_hocon_value(uri, value));
            }

            Value::new(uri, ValueKind::Table(m))
        }

        hocon::Hocon::Array(ref array) => {
            let mut l = Vec::new();

            for value in array {
                l.push(from_hocon_value(uri, value));
            }

            Value::new(uri, ValueKind::Array(l))
        }

        hocon::Hocon::Null | hocon::Hocon::BadValue(_) => Value::new(uri, ValueKind::Nil),
    }
}

use crate::map::Map;
use std::error::Error;

use crate::{
    error::Unexpected,
    value::{Value, ValueKind},
    ConfigError,
};

pub fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
    let value = from_dhall_value(uri, serde_dhall::from_str(text).parse()?);
    match value.kind {
        ValueKind::Table(map) => Ok(map),
        ValueKind::Nil => Err(Unexpected::Unit),
        ValueKind::Boolean(value) => Err(Unexpected::Bool(value)),
        ValueKind::Integer(value) => Err(Unexpected::Integer(value)),
        ValueKind::Float(value) => Err(Unexpected::Float(value)),
        ValueKind::String(value) => Err(Unexpected::Str(value)),
        ValueKind::Array(value) => Err(Unexpected::Seq),
    }
    .map_err(|err| ConfigError::invalid_root(uri, err))
    .map_err(|err| Box::new(err) as Box<dyn Error + Send + Sync>)
}

fn from_dhall_value(uri: Option<&String>, value: serde_dhall::SimpleValue) -> Value {
    match value {
        serde_dhall::SimpleValue::Num(num) => match num {
            serde_dhall::NumKind::Bool(b) => Value::new(uri, ValueKind::Boolean(b)),
            serde_dhall::NumKind::Natural(n) => Value::new(uri, ValueKind::Integer(n as i64)),
            serde_dhall::NumKind::Integer(i) => Value::new(uri, ValueKind::Integer(i)),
            serde_dhall::NumKind::Double(d) => Value::new(uri, ValueKind::Float(f64::from(d))),
        },
        serde_dhall::SimpleValue::Text(string) => Value::new(uri, ValueKind::String(string)),
        serde_dhall::SimpleValue::List(list) => Value::new(
            uri,
            ValueKind::Array(list.into_iter().map(|v| from_dhall_value(uri, v)).collect()),
        ),
        serde_dhall::SimpleValue::Record(rec) => Value::new(
            uri,
            ValueKind::Table(
                rec.into_iter()
                    .map(|(k, v)| (k, from_dhall_value(uri, v)))
                    .collect(),
            ),
        ),
        serde_dhall::SimpleValue::Optional(Some(value))
        | serde_dhall::SimpleValue::Union(_, Some(value)) => from_dhall_value(uri, *value),
        serde_dhall::SimpleValue::Optional(None) | serde_dhall::SimpleValue::Union(_, None) => {
            Value::new(uri, ValueKind::Nil)
        }
    }
}
